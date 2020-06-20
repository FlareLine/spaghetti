use crate::schema::recipe;
use diesel::{PgConnection, Queryable, QueryDsl, RunQueryDsl, ExpressionMethods};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct ThinRecipe {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub archived: bool,
}

impl ThinRecipe {
    pub fn find(id: &i32, connection: &PgConnection) -> Result<ThinRecipe, diesel::result::Error> {
        recipe::table
            .find(id)
            .select((
                recipe::id,
                recipe::name,
                recipe::description,
                recipe::image_url,
                recipe::archived,
            ))
            .first(connection)
    }
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "recipe"]
pub struct NewRecipe {
    pub name: String,
    pub description: String,
    pub image_url: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub archived: bool,
}

impl From<Recipe> for ThinRecipe {
    fn from(rcp: Recipe) -> ThinRecipe {
        ThinRecipe {
            id: rcp.id,
            name: rcp.name,
            description: rcp.description,
            image_url: rcp.image_url,
            archived: rcp.archived,
        }
    }
}

impl Recipe {
    pub fn create(
        rcp: &NewRecipe,
        connection: &PgConnection,
    ) -> Result<ThinRecipe, diesel::result::Error> {
        use crate::schema::recipe::dsl::*;

        diesel::insert_into(recipe)
            .values(rcp)
            .get_result::<Recipe>(connection)
            .map(Into::into)
    }

    pub fn update(
        recipe_id: i32,
        recipe_data: &NewRecipe,
        connection: &PgConnection,
    ) -> Result<ThinRecipe, diesel::result::Error> {
        use crate::schema::recipe::dsl::*;

        let recipe_name = &recipe_data.name;
        let recipe_description = &recipe_data.description;
        let recipe_image = &recipe_data.image_url;

        diesel::update(recipe.filter(id.eq(recipe_id)))
            .set((
                name.eq(recipe_name),
                description.eq(recipe_description),
                image_url.eq(recipe_image),
            ))
            .get_result(connection)
    }

    pub fn archive(
        recipe_id: i32,
        connection: &PgConnection
    ) -> Result<ThinRecipe, diesel::result::Error> {
        use crate::schema::recipe::dsl::*;

        diesel::update(recipe.filter(id.eq(recipe_id)))
            .set(
                archived.eq(true)
            )
            .get_result(connection)
    }
}

pub mod ingredient;