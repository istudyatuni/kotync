use rocket::get;

#[get("/")]
pub fn root() -> &'static str {
	"Alive"
}
