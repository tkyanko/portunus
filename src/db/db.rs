use diesel::prelude::*;
use diesel::sql_query;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use time::ext::NumericalDuration;
use time::{OffsetDateTime, PrimitiveDateTime};

use crate::db::models::{Key, KeyCount, NewKey, NewProduct, NewUser, Product, User};
use crate::db::schema;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn add_key(game_name: &String, new_product_key: &String, user_discord_id: &String) {
    use crate::db::schema::products::dsl::*;
    use crate::db::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let product_id_for_new_key = match products
        .filter(title.eq(game_name))
        .first::<Product>(connection)
    {
        Ok(product) => product.id,
        Err(_) => {
            let new_product = NewProduct { title: game_name };
            diesel::insert_into(schema::products::table)
                .values(&new_product)
                .on_conflict_do_nothing()
                .get_result::<Product>(connection)
                .unwrap()
                .id
        }
    };

    let user_id_for_new_key = match users
        .filter(discord_id.eq(user_discord_id))
        .first::<User>(connection)
    {
        Ok(user) => {
            let user_keys_given = user.keys_given + 1;
            diesel::update(users.filter(schema::users::id.eq(user.id)))
                .set(keys_given.eq(user_keys_given))
                .execute(connection)
                .expect("Could not update key during claiming");
            user.id
        }
        Err(_) => {
            let new_user = NewUser {
                discord_id: user_discord_id,
                keys_given: &1,
            };
            diesel::insert_into(schema::users::table)
                .values(&new_user)
                .on_conflict_do_nothing()
                .get_result::<User>(connection)
                .unwrap()
                .id
        }
    };

    let now_utc = OffsetDateTime::now_utc();
    let now = PrimitiveDateTime::new(now_utc.date(), now_utc.time());

    let new_key = NewKey {
        product_id: &product_id_for_new_key,
        product_key: new_product_key,
        time_added: &now,
        user_who_added: &user_id_for_new_key,
    };

    diesel::insert_into(schema::keys::table)
        .values(&new_key)
        .execute(connection)
        .expect("Error inserting key into database");
}

pub fn get_key(game_name: &String, user_discord_id: &String) -> Result<String, String> {
    use crate::db::schema::keys::dsl::*;
    use crate::db::schema::products::dsl::*;
    use crate::db::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let now_utc = OffsetDateTime::now_utc();
    let now = PrimitiveDateTime::new(now_utc.date(), now_utc.time());

    let mut user = match users
        .filter(discord_id.eq(user_discord_id))
        .first::<User>(connection)
    {
        Ok(user) => user,
        Err(_) => {
            let new_user = NewUser {
                discord_id: user_discord_id,
                keys_given: &0,
            };
            diesel::insert_into(schema::users::table)
                .values(&new_user)
                .get_result::<User>(connection)
                .unwrap()
        }
    };

    // Make sure the user has not redeemed a key within the last 24 hours.
    if user.last_taken_time > now.checked_sub(1.days()).expect("Invalid current time.") {
        return Err(String::from(
            "Sorry, but you must wait at least one day between claiming keys.",
        ));
    }

    let product = match products
        .filter(title.eq(game_name))
        .first::<Product>(connection)
    {
        Ok(product) => product.id,
        Err(_) => {
            return Err(String::from(
                "Sorry, but that game has never been entered into the database.",
            ));
        }
    };

    match keys
        .filter(product_id.eq(product))
        .filter(user_who_claimed.is_null())
        .first::<Key>(connection)
    {
        Ok(key) => {
            diesel::insert_into(schema::keys::table)
                .values(&key)
                .on_conflict(schema::keys::id)
                .do_update()
                .set((
                    time_claimed.eq(Some(now)),
                    user_who_claimed.eq(Some(user.id)),
                ))
                .execute(connection)
                .expect("Could not update key during claiming");
            user.keys_taken += 1;
            diesel::insert_into(schema::users::table)
                .values(&user)
                .on_conflict(schema::users::id)
                .do_update()
                .set((last_taken_time.eq(now), keys_taken.eq(user.keys_taken)))
                .execute(connection)
                .expect("Could not update user during claiming");
            Ok(key.product_key.to_string())
        }
        Err(_) => Err("Sorry, there are not keys available for that game.".to_string()),
    }
}

pub fn list_keys() -> Result<Vec<KeyCount>, String> {
    let connection = &mut establish_connection();
    match sql_query(
        "SELECT title, count(title) AS count
FROM keys JOIN products p on p.id = keys.product_id
WHERE user_who_claimed IS NULL
GROUP BY title",
    )
    .load::<KeyCount>(connection)
    {
        Ok(key_counts) => Ok(key_counts),
        Err(_) => Err("Error loading key counts".parse().unwrap()),
    }
}
