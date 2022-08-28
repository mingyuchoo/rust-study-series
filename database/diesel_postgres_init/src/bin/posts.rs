use diesel_postgres_init::*;

fn main() {
    let connection = &mut establish_connection();

    let _new_post = create_post(connection, "Hello", "World!", true);
    let _num_deleted = delete_post(connection, "Hello");
    let _removed_post = update_post(connection, 1);

    show_posts(connection);
}
