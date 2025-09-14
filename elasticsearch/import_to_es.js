// import_to_es.js
import { Client } from '@elastic/elasticsearch';
import mysql from 'mysql2/promise';

const esClient = new Client({
  node: 'https://localhost:9200',
  auth: { username: 'elastic', password: 'xlhuHK-gHz+O8O1W0CWV' },
  tls: { rejectUnauthorized: false }
});

async function createIndex() {
  await esClient.indices.create({
    index: 'vanban',
    body: {
      settings: {
        analysis: {
          analyzer: {
            vietnamese_analyzer: {
              tokenizer: 'standard',
              filter: ['lowercase', 'asciifolding']
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
  }, { ignore: [400] });
}

async function importFromMySQL() {
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

  const body = rows.flatMap(doc => [
    { index: { _index: 'vanban', _id: doc.filecode } },
    doc
  ]);

  await esClient.bulk({ refresh: true, body });
  console.log('✅ Import completed!');

  await connection.end();
}

(async () => {
  await createIndex();
  await importFromMySQL();
})();
