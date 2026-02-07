// [REQ-N002] 샘플 시드 데이터 생성 태스크 (2026-02-07)
use loco_rs::prelude::*;
use sea_orm::ActiveValue;

use crate::models::_entities::{comments, tracks, users};

// [REQ-N006] 실제 YouTube 영상 정보 통합 배열: (youtube_id, title, artist, genre)
const TRACK_DATA: [(&str, &str, &str, &str); 100] = [
    ("kJQP7kiw5Fk", "Despacito", "Luis Fonsi ft. Daddy Yankee", "Latin Pop"),
    ("RgKAFK5djSk", "See You Again", "Wiz Khalifa ft. Charlie Puth", "Hip-Hop"),
    ("JGwWNGJdvx8", "Shape of You", "Ed Sheeran", "Pop"),
    ("9bZkp7q19f0", "Gangnam Style", "PSY", "K-Pop"),
    ("OPf0YbXqDm0", "Uptown Funk", "Mark Ronson ft. Bruno Mars", "Funk"),
    ("pRpeEdMmmQ0", "Waka Waka", "Shakira", "Pop"),
    ("hT_nvWreIhg", "Counting Stars", "OneRepublic", "Pop Rock"),
    ("09R8_2nJtjg", "Sugar", "Maroon 5", "Pop"),
    ("CevxZvSJLk8", "Roar", "Katy Perry", "Pop"),
    ("0KSOMA3QBU0", "Dark Horse", "Katy Perry ft. Juicy J", "Pop"),
    ("2Vv-BfVoq4g", "Perfect", "Ed Sheeran", "Pop"),
    ("fRh_vgS2dFE", "Sorry", "Justin Bieber", "Pop"),
    ("RBumgq5yVrA", "Let Her Go", "Passenger", "Folk Pop"),
    ("lp-EO5I60KA", "Thinking Out Loud", "Ed Sheeran", "Pop"),
    ("aJOTlE1K90k", "Girls Like You", "Maroon 5 ft. Cardi B", "Pop"),
    ("60ItHLz5WEA", "Faded", "Alan Walker", "Electronic"),
    ("YqeW9_5kURI", "Lean On", "Major Lazer & DJ Snake ft. MØ", "Electronic"),
    ("NUsoVlDFqZg", "Bailando", "Enrique Iglesias", "Latin Pop"),
    ("e-ORhEE9VVg", "Blank Space", "Taylor Swift", "Pop"),
    ("nfWlot6h_JM", "Shake It Off", "Taylor Swift", "Pop"),
    ("L0MK7qz13bU", "Let It Go", "Idina Menzel", "Pop"),
    ("wnJ6LuUFpMo", "Mi Gente", "J Balvin & Willy William", "Reggaeton"),
    ("kffacxfA7G4", "Baby", "Justin Bieber ft. Ludacris", "Pop"),
    ("3AtDnEC4zak", "We Don't Talk Anymore", "Charlie Puth ft. Selena Gomez", "Pop"),
    ("PT2_F-1esPk", "Closer", "The Chainsmokers ft. Halsey", "Electronic"),
    ("YQHsXMglC9A", "Hello", "Adele", "Pop"),
    ("k2qgadSvNyU", "New Rules", "Dua Lipa", "Pop"),
    ("pXRviuL6vMY", "Stressed Out", "Twenty One Pilots", "Alternative"),
    ("papuvlVeZg8", "Rockabye", "Clean Bandit ft. Sean Paul & Anne-Marie", "Electronic"),
    ("fLexgOxsZu0", "The Lazy Song", "Bruno Mars", "Pop"),
    ("6Mgqbai3fKo", "Chantaje", "Shakira ft. Maluma", "Reggaeton"),
    ("DiItGE3eAyQ", "Con Calma", "Daddy Yankee ft. Snow", "Reggaeton"),
    ("1_zgKRBrT0Y", "Calma", "Pedro Capo & Farruko", "Latin Pop"),
    ("uelHwf8o7_U", "Love The Way You Lie", "Eminem ft. Rihanna", "Hip-Hop"),
    ("5GL9JoH4Sws", "Work from Home", "Fifth Harmony ft. Ty Dolla Sign", "Pop"),
    ("7wtfhZwyrcc", "Believer", "Imagine Dragons", "Rock"),
    ("kOkQ4T5WO9E", "This Is What You Came For", "Calvin Harris ft. Rihanna", "Electronic"),
    ("2vjPBrBU-TM", "Chandelier", "Sia", "Pop"),
    ("ApXoWvfEYVU", "Sunflower", "Post Malone & Swae Lee", "Hip-Hop"),
    ("rYEDA3JcQqw", "Rolling in the Deep", "Adele", "Pop"),
    ("VqEbCxg2bNI", "Criminal", "Natti Natasha x Ozuna", "Reggaeton"),
    ("ixkoVwKQaJg", "Taki Taki", "DJ Snake ft. Selena Gomez, Ozuna & Cardi B", "Reggaeton"),
    ("rtOvBOTyX00", "A Thousand Years", "Christina Perri", "Pop"),
    ("t4H_Zoh7G5A", "On The Floor", "Jennifer Lopez ft. Pitbull", "Pop"),
    ("34Na4j8AVgA", "Starboy", "The Weeknd ft. Daft Punk", "R&B"),
    ("7PCkvCPvDXk", "All About That Bass", "Meghan Trainor", "Pop"),
    ("ALZHF5UqnU4", "Alone", "Marshmello", "Electronic"),
    ("lY2yjAdbvdQ", "Treat You Better", "Shawn Mendes", "Pop"),
    ("kXYiU_JCYtU", "Numb", "Linkin Park", "Rock"),
    ("450p7goxZqg", "All of Me", "John Legend", "R&B"),
    ("PIh2xe4jnpk", "Rude", "MAGIC!", "Reggae Pop"),
    ("GMFewiplIbw", "Mayores", "Becky G ft. Bad Bunny", "Reggaeton"),
    ("8UVNT4wvIGY", "Somebody That I Used to Know", "Gotye ft. Kimbra", "Indie Pop"),
    ("5qm8PH4xAss", "In Da Club", "50 Cent", "Hip-Hop"),
    ("9jI-z9QN6g8", "Te Bote Remix", "Nio Garcia, Casper Magico & others", "Reggaeton"),
    ("w2C6RhQBYlg", "No Me Conoce", "Jhay Cortez, J Balvin & Bad Bunny", "Reggaeton"),
    ("KQ6zr6kCPj8", "Party Rock Anthem", "LMFAO", "Electronic"),
    ("lWA2pjMjpBs", "Diamonds", "Rihanna", "Pop"),
    ("IcrbM1l_BoI", "Wake Me Up", "Avicii", "Electronic"),
    ("PMivT7MJ41M", "That's What I Like", "Bruno Mars", "R&B"),
    ("TyHvyGVs42U", "Echame La Culpa", "Luis Fonsi & Demi Lovato", "Latin Pop"),
    ("FM7MFYoylVs", "Something Just Like This", "The Chainsmokers & Coldplay", "Electronic"),
    ("hLQl3WQQoQ0", "Someone Like You", "Adele", "Pop"),
    ("YBHQbu5rbdQ", "Worth It", "Fifth Harmony ft. Kid Ink", "Pop"),
    ("AJtDXIazrMo", "Love Me Like You Do", "Ellie Goulding", "Pop"),
    ("8SbUC-UaAxE", "November Rain", "Guns N' Roses", "Rock"),
    ("IHNzOHi8sJs", "DDU-DU DDU-DU", "BLACKPINK", "K-Pop"),
    ("V1Pl8CzNzCw", "Lovely", "Billie Eilish & Khalid", "Pop"),
    ("fKopy74weus", "Thunder", "Imagine Dragons", "Pop Rock"),
    ("tt2k8PGm-TI", "Dusk Till Dawn", "ZAYN ft. Sia", "Pop"),
    ("k76BgIb89-s", "Nunca Es Suficiente", "Los Angeles Azules ft. Natalia Lafourcade", "Latin Pop"),
    ("QFs3PIZb3js", "Propuesta Indecente", "Romeo Santos", "Bachata"),
    ("djV11Xbc914", "Take On Me", "a-ha", "Pop"),
    ("YVkUvmDQ3HY", "Without Me", "Eminem", "Hip-Hop"),
    ("UprcpdwuwCg", "Heathens", "Twenty One Pilots", "Alternative"),
    ("_I_D_8Z4sJE", "X (Equis)", "Nicky Jam x J Balvin", "Reggaeton"),
    ("ekr2nIex040", "APT.", "ROSE & Bruno Mars", "Pop"),
    ("zEf423kYfqk", "Sin Pijama", "Becky G & Natti Natasha", "Reggaeton"),
    ("0VR3dfZf9Yg", "China", "Anuel AA, Daddy Yankee, Karol G, Ozuna & J Balvin", "Reggaeton"),
    ("DK_0jXPuIr0", "What Do You Mean?", "Justin Bieber", "Pop"),
    ("Io0fBr1XBUA", "Don't Let Me Down", "The Chainsmokers ft. Daya", "Electronic"),
    ("q0hyYWKXF0Q", "Dance Monkey", "Tones and I", "Pop"),
    ("yzTuBuRdAyA", "The Hills", "The Weeknd", "R&B"),
    ("p7bfOZek9t4", "Con Altura", "ROSALIA & J Balvin ft. El Guincho", "Reggaeton"),
    ("l0U7SxXHkPY", "Life Is Good", "Future ft. Drake", "Hip-Hop"),
    ("FzG4uDgje3M", "Dame Tu Cosita", "El Chombo ft. Cutty Ranks", "Reggaeton"),
    ("YykjpeuMNEk", "Hymn for the Weekend", "Coldplay", "Pop"),
    ("eVTXPUF4Oz4", "In The End", "Linkin Park", "Rock"),
    ("2S24-y0Ij3Y", "Kill This Love", "BLACKPINK", "K-Pop"),
    ("L3wKzyIN1yk", "Human", "Rag'n'Bone Man", "Pop"),
    ("LjhCEhWiKXk", "Just The Way You Are", "Bruno Mars", "Pop"),
    ("hTWKbfoikeg", "Smells Like Teen Spirit", "Nirvana", "Rock"),
    ("uxpDa-c-4Mc", "Hotline Bling", "Drake", "R&B"),
    ("0HDdjwpPM3Y", "Bang Bang", "Jessie J, Ariana Grande & Nicki Minaj", "Pop"),
    ("JRfuAukYTKg", "Titanium", "David Guetta ft. Sia", "Electronic"),
    ("6NXnxTNIWkc", "What's Up", "4 Non Blondes", "Rock"),
    ("sGIm0-dQd8M", "Dura", "Daddy Yankee", "Reggaeton"),
    ("XXYlFuWEuKI", "Save Your Tears", "The Weeknd", "Pop"),
    ("HCjNJDNzw8Y", "Havana", "Camila Cabello ft. Young Thug", "Pop"),
    ("j5-yKhDd64s", "Not Afraid", "Eminem", "Hip-Hop"),
];

const COMMENT_TEMPLATES: [&str; 20] = [
    "Great track! Love the vibes.",
    "This is amazing, keep it up!",
    "Perfect for a late night drive.",
    "Incredible sound quality!",
    "This track is on repeat!",
    "Love the melody, so catchy!",
    "One of my favorites now.",
    "The rhythm is fantastic!",
    "Brilliant composition!",
    "Can't stop listening to this.",
    "This deserves more attention!",
    "What an awesome track!",
    "The bass line is killer!",
    "Pure musical genius.",
    "So relaxing and beautiful.",
    "This track makes my day!",
    "Absolute banger!",
    "The vocals are incredible.",
    "Best track I've heard this week!",
    "Such a unique sound!",
];

pub struct SeedData;

#[async_trait]
impl Task for SeedData {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "seed_data".to_string(),
            detail: "Seed 100 sample users, 100 tracks, and 100 comments".to_string(),
        }
    }

    // [REQ-N002] 100명의 회원, 100개의 트랙, 100개의 댓글을 생성한다
    async fn run(&self, ctx: &AppContext, _vars: &task::Vars) -> Result<()> {
        let db = &ctx.db;

        // 비밀번호 해시를 1회만 계산하여 재사용 (Argon2id 성능 최적화)
        let password_hash =
            loco_rs::hash::hash_password("password123").map_err(loco_rs::Error::wrap)?;

        // Step 1: 회원 100명 생성
        tracing::info!("Creating 100 sample users...");
        let mut user_ids = Vec::with_capacity(100);
        for i in 1..=100u32 {
            let user = users::ActiveModel {
                email: ActiveValue::Set(format!("user{i}@example.com")),
                password: ActiveValue::Set(password_hash.clone()),
                name: ActiveValue::Set(format!("User {i}")),
                ..Default::default()
            }
            .insert(db)
            .await?;
            user_ids.push(user.id);
        }
        tracing::info!("Created {} users", user_ids.len());

        // Step 2: 트랙 100개 생성 (모두 공개 상태)
        // [REQ-N006] 실제 YouTube 영상 정보(제목, 아티스트, 장르) 사용
        tracing::info!("Creating 100 sample tracks...");
        let mut track_ids = Vec::with_capacity(100);
        for i in 1..=100u32 {
            let idx = (i - 1) as usize;
            let (youtube_id, title, artist, genre) = TRACK_DATA[idx];
            let user_id = user_ids[idx % user_ids.len()];

            let track = tracks::ActiveModel {
                user_id: ActiveValue::Set(user_id),
                title: ActiveValue::Set(title.to_string()),
                artist: ActiveValue::Set(Some(artist.to_string())),
                url: ActiveValue::Set(format!(
                    "https://www.youtube.com/watch?v={youtube_id}"
                )),
                description: ActiveValue::Set(Some(format!(
                    "{genre} hit by {artist}"
                ))),
                is_public: ActiveValue::Set(true),
                vote_score: ActiveValue::Set(0),
                ..Default::default()
            }
            .insert(db)
            .await?;
            track_ids.push(track.id);
        }
        tracing::info!("Created {} tracks", track_ids.len());

        // Step 3: 공개 트랙에 댓글 100개 생성 (트랙 소유자와 다른 사용자가 작성)
        tracing::info!("Creating 100 sample comments...");
        for i in 1..=100u32 {
            let idx = (i - 1) as usize;
            let track_id = track_ids[idx % track_ids.len()];
            let user_id = user_ids[(idx + 7) % user_ids.len()];
            let content = COMMENT_TEMPLATES[idx % COMMENT_TEMPLATES.len()];

            comments::ActiveModel {
                track_id: ActiveValue::Set(track_id),
                user_id: ActiveValue::Set(user_id),
                content: ActiveValue::Set(content.to_string()),
                ..Default::default()
            }
            .insert(db)
            .await?;
        }
        tracing::info!("Created 100 comments");

        tracing::info!(
            "Seed data created successfully: 100 users, 100 tracks, 100 comments"
        );
        Ok(())
    }
}
