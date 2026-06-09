use argon2::{
password_hash::{
rand_core::OsRng,
PasswordHasher,
SaltString,
},
Argon2,
};

use sqlx::PgPool;

pub async fn seed_default_user(pool: &PgPool) -> Result<(), sqlx::Error> {

```
// Check if user already exists
let existing = sqlx::query!(
    r#"
    SELECT id
    FROM users
    WHERE email = $1
    "#,
    "janon@lkprofessionals.com"
)
.fetch_optional(pool)
.await?;

if existing.is_some() {
    println!("Default admin user already exists.");
    return Ok(());
}

// Hash password
let salt = SaltString::generate(&mut OsRng);

let password_hash = Argon2::default()
    .hash_password("Jj112112@!@!".as_bytes(), &salt)
    .unwrap()
    .to_string();

// Insert user
sqlx::query!(
    r#"
    INSERT INTO users (
        name,
        email,
        password_hash,
        role,
        is_active,
        email_verified_at
    )
    VALUES ($1, $2, $3, $4, $5, NOW())
    "#,
    "janon-emersion-t",
    "janon@lkprofessionals.com",
    password_hash,
    "super_admin",
    true
)
.execute(pool)
.await?;

println!("Default admin user seeded successfully.");

Ok(())
```

}
