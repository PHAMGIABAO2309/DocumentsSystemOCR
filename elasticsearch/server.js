// app.js
import express from 'express';
import cors from 'cors';
import { Client } from '@elastic/elasticsearch';
import mysql from 'mysql2/promise';

const app = express();
app.use(cors());

// Elasticsearch client
const esClient = new Client({
  node: 'https://localhost:9200',
  auth: { username: 'elastic', password: 'xlhuHK-gHz+O8O1W0CWV' },
  tls: { rejectUnauthorized: false }
});

// ===== 1. Tạo index nếu chưa có (dùng ICU analyzer cho tiếng Việt) =====
async function createIndex() {
  await esClient.indices.create({
    index: 'vanban',
    body: {
      settings: {
        analysis: {
          analyzer: {
            vietnamese_analyzer: {
              tokenizer: 'icu_tokenizer',
              filter: ['lowercase', 'icu_folding']
            }
          }
        }
      },
      mappings: {
        properties: {
          filecode: { type: 'keyword' },
          title: { type: 'text', analyzer: 'vietnamese_analyzer' },
          startdate: { type: 'date' },
          dateupdate: { type: 'date' },
          path: { type: 'keyword' }
        }
      }
    }
  }, { ignore: [400] }); // ignore nếu index đã tồn tại
  console.log('✅ Index created or exists.');
}

// ===== 2. Import dữ liệu từ MySQL theo batches =====
async function importFromMySQL(batchSize = 500) {
  const connection = await mysql.createConnection({
    host: 'localhost',
    user: 'root',
    password: '',
    database: 'storages_documents_3'
  });

  const [rows] = await connection.execute(`
    SELECT FileCode AS filecode, Title AS title, StartDate AS startdate, dateupdate, path
    FROM files
  `);

  console.log(`📦 Importing ${rows.length} documents into Elasticsearch...`);

  for (let i = 0; i < rows.length; i += batchSize) {
    const batch = rows.slice(i, i + batchSize);
    const body = batch.flatMap(doc => [{ index: { _index: 'vanban', _id: doc.filecode } }, doc]);
    await esClient.bulk({ refresh: true, body });
    console.log(`✅ Imported batch ${i / batchSize + 1}`);
  }

  await connection.end();
  console.log('✅ Import completed!');
}

// ===== 3. API Search + Suggestion =====
app.get('/api/search_suggest', async (req, res) => {
  const q = (req.query.titles || '').trim();
  if (!q) return res.status(400).json({ error: 'Vui lòng nhập từ khóa' });

  const keywords = q.split(/\s+/).filter(Boolean);
  const results = [];

  try {
    // Search lần đầu
    let response = await esClient.search({
      index: 'vanban',
      scroll: '1m',
      size: 1000,
      track_total_hits: true,
      _source: ['filecode', 'title'],
      query: {
        bool: {
          must: keywords.map(word => ({
            match_phrase_prefix: { title: word } // tốt cho suggestion
          }))
        }
      },
      highlight: {
        fields: { title: {} },
        pre_tags: ['<em>'],
        post_tags: ['</em>']
      }
    });

    let scrollId = response._scroll_id;

    while (response.hits.hits.length > 0) {
      results.push(
        ...response.hits.hits.map(hit => ({
          filecode: hit._source.filecode,
          title: hit.highlight?.title?.[0] || hit._source.title
        }))
      );

      response = await esClient.scroll({ scroll_id: scrollId, scroll: '1m' });
      scrollId = response._scroll_id;
    }

    // Clear scroll để tránh tốn resource
    if (scrollId) await esClient.clearScroll({ scroll_id: scrollId });

    res.setHeader('Content-Type', 'application/json; charset=utf-8');
    res.send(JSON.stringify(results, null, 2));
  } catch (err) {
    console.error(err);
    res.status(500).json({ error: err.message });
  }
});

// ===== 4. Chạy server =====
(async () => {
  await createIndex();
  await importFromMySQL();
  app.listen(5000, () => console.log('✅ Server running at http://localhost:5000'));
})();
