use actix_web::{web, App, HttpServer};
use actix_cors::Cors; 
use crate::configs::connect::ketnoi_cosodulieu;

use actix_files::Files;

use crate::api::hiennguoidung::test::api_json_user;
use crate::api::trangchu::api_trangchu::api_json_trangchu_danhsachvanban;
use crate::api::dinhkemfile::api_dinhkemfile::get_dinhkemfile_json;
use crate::api::vanban::api_vanban::api_json_vanban;
use crate::api::tomtatvanban::api_tomtat::api_json_tomtat;
use crate::api::goiytimkiem::api_goiytimkiem::api_json_goiytimkiem;
use crate::api::thuoctinhvanban::api_thuoctinhvanban::api_json_thuoctinhvanban;
use crate::api::timtheolinhvuc::api_timtheolinhvuc::api_json_timtheolinhvuc;
use crate::api::timtheonambanhanh::api_timtheonam::api_json_timtheonambanhanh;
use crate::api::timtheoloaivanban::api_timtheoloaivanban::api_json_timtheoloaivanban;
use crate::api::timtheocoquanbanhanh::api_timtheocoquan::api_json_timtheocoquan;
use crate::api::dangnhap::api_dangnhap::post_login;
use crate::api::dangky::api_dangky::post_register;
use crate::api::quenmatkhau::api_quenmatkhau::post_quenmatkhau;
use crate::api::xacnhanmatkhau::api_xacnhanmatkhau::update_password;

use crate::api::admin_danhsachvanban::api_danhsachvanban::get_danhsachvanban_json;
use crate::api::admin_quanlyvanbanden::api_quanlyvanbanden::{
   post_admin_themvanbanden, delete_vanbanden, update_vanbanden,
};
use crate::api::admin_thuoctinhvanban::api_thuoctinhvanban::get_admin_article_data;
use crate::api::admin_qlvbocr::api_qlvbocr::post_admin_quanlyvanbanocr;
use crate::api::admin_quanlytaikhoan::api_admin_account::{get_account_json, update_account, delete_account};
use crate::api::admin_quanlyloaivanban::api_quanlyloaivanban::{get_listtype_json, post_admin_add_typedocuments, check_typedocuments_typeid,update_typedocument,delete_typedocument};
use crate::api::admin_quanlycoquan::api_admin_quanlycoquan::{get_organization_json, get_new_organization_code, post_admin_add_organization,update_organization,delete_organization};

pub async fn start_server() -> std::io::Result<()> {
    let (pool, address) = ketnoi_cosodulieu().await;

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())

            .app_data(web::Data::new(pool.clone()))

            .service(api_json_user)
            .service(api_json_trangchu_danhsachvanban)
            .service(get_dinhkemfile_json)
            .service(api_json_vanban)
            .service(api_json_tomtat)
            .service(api_json_goiytimkiem)
            .service(api_json_thuoctinhvanban)
            .service(api_json_timtheolinhvuc)
            .service(api_json_timtheonambanhanh)
            .service(api_json_timtheoloaivanban)
            .service(api_json_timtheocoquan)
            .service(post_login)
            .service(post_register)
            .service(post_quenmatkhau)
            .service(update_password)

            .service(get_danhsachvanban_json)

            .service(post_admin_themvanbanden)
            .service(delete_vanbanden)
            .service(update_vanbanden)

            .service(get_admin_article_data)
            .service(post_admin_quanlyvanbanocr)
            .service(get_account_json)
            .service(update_account)
            .service(delete_account)

            .service(get_listtype_json)
            .service(post_admin_add_typedocuments)
            .service(check_typedocuments_typeid)
            .service(update_typedocument)
            .service(delete_typedocument)

            .service(get_organization_json)
            .service(get_new_organization_code)
            .service(post_admin_add_organization)
            .service(update_organization)
            .service(delete_organization)

            .service(Files::new("/static", "./static").show_files_listing())

    })
    .bind(&address)?
    .run()
    .await
}
