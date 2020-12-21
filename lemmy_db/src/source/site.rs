use crate::Crud;
use diesel::{dsl::*, result::Error, *};
use lemmy_db_schema::{naive_now, source::site::*};

impl Crud<SiteForm> for Site {
  fn read(conn: &PgConnection, _site_id: i32) -> Result<Self, Error> {
    use lemmy_db_schema::schema::site::dsl::*;
    site.first::<Self>(conn)
  }

  fn create(conn: &PgConnection, new_site: &SiteForm) -> Result<Self, Error> {
    use lemmy_db_schema::schema::site::dsl::*;
    insert_into(site).values(new_site).get_result::<Self>(conn)
  }

  fn update(conn: &PgConnection, site_id: i32, new_site: &SiteForm) -> Result<Self, Error> {
    use lemmy_db_schema::schema::site::dsl::*;
    diesel::update(site.find(site_id))
      .set(new_site)
      .get_result::<Self>(conn)
  }
}

pub trait Site_ {
  fn transfer(conn: &PgConnection, new_creator_id: i32) -> Result<Site, Error>;
}

impl Site_ for Site {
  fn transfer(conn: &PgConnection, new_creator_id: i32) -> Result<Site, Error> {
    use lemmy_db_schema::schema::site::dsl::*;
    diesel::update(site.find(1))
      .set((creator_id.eq(new_creator_id), updated.eq(naive_now())))
      .get_result::<Self>(conn)
  }
}
