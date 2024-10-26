use self::models::*;
use diesel::prelude::*;
use diesel_mysql_init::*;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    use self::schema::posts::dsl::*;

    let connection: &mut MysqlConnection = &mut establish_connection();
    let results = posts.filter(published.eq(true))
                       .limit(5)
                       .select(Post::as_select())
                       .load(connection)
                       .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }

    Ok(())
}
