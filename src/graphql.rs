use juniper::{FieldResult, RootNode};

use crate::db::Context;
use crate::models::graphql::Recipe;
use crate::repository::RecipeRepository;

pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn recipe_by_name(ctx: &Context, name: String) -> FieldResult<Option<Recipe>> {
        RecipeRepository::get_recipe_by_name(&ctx.pool.get().unwrap(), &name)
    }

    fn recipes_by_ingredient(ctx: &Context, name: String) -> FieldResult<Vec<Recipe>> {
        RecipeRepository::get_recipes_by_ingredient_name(&ctx.pool.get().unwrap(), &name)
    }

    fn recipes_by_ingredients(ctx: &Context, names: Vec<String>) -> FieldResult<Vec<Recipe>> {
        RecipeRepository::get_recipes_by_ingredient_names(&ctx.pool.get().unwrap(), &names)
    }
}

pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
}

pub fn schema() -> RootNode<'static, Query, Mutation> {
    juniper::RootNode::new(Query, Mutation)
}