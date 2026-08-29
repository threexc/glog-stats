use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use glog_v2_character_generator::{CharacterGenerator, Character};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::services::ServeDir;

type AppState = Arc<CharacterGenerator>;

#[derive(Debug, Deserialize)]
struct GenerateRequest {
    level: u32,
    count: u32,
}

#[derive(Debug, Serialize)]
struct GenerateResponse {
    characters: Vec<Character>,
    success: bool,
    message: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the character generator
    let generator = CharacterGenerator::new("config.toml")?;
    let app_state = Arc::new(generator);

    // Build our application with routes
    let app = Router::new()
        .route("/", get(serve_index))
        .route("/generate", post(generate_characters))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(app_state);

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("🎲 GLOG v2 Character Generator (Web Server)");
    println!("======================================");
    println!("Server running on http://localhost:3000");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn serve_index() -> Html<String> {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>GLOG v2 Character Generator</title>
    <style>
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            color: #333;
        }
        
        .container {
            background: white;
            border-radius: 15px;
            padding: 30px;
            box-shadow: 0 10px 30px rgba(0,0,0,0.3);
        }
        
        h1 {
            text-align: center;
            color: #4a5568;
            margin-bottom: 30px;
            font-size: 2.5em;
            text-shadow: 2px 2px 4px rgba(0,0,0,0.1);
        }
        
        .form-group {
            margin-bottom: 20px;
        }
        
        label {
            display: block;
            margin-bottom: 8px;
            font-weight: 600;
            color: #2d3748;
        }
        
        input {
            width: 100%;
            padding: 12px;
            border: 2px solid #e2e8f0;
            border-radius: 8px;
            font-size: 16px;
            transition: border-color 0.3s;
        }
        
        input:focus {
            outline: none;
            border-color: #667eea;
            box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
        }
        
        button {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            padding: 15px 30px;
            border-radius: 8px;
            font-size: 18px;
            font-weight: 600;
            cursor: pointer;
            width: 100%;
            transition: transform 0.2s, box-shadow 0.2s;
        }
        
        button:hover {
            transform: translateY(-2px);
            box-shadow: 0 5px 15px rgba(102, 126, 234, 0.4);
        }
        
        button:disabled {
            opacity: 0.6;
            cursor: not-allowed;
            transform: none;
        }
        
        .character {
            background: #f7fafc;
            border: 2px solid #e2e8f0;
            border-radius: 10px;
            padding: 20px;
            margin: 15px 0;
            transition: transform 0.2s;
        }
        
        .character:hover {
            transform: translateY(-2px);
            box-shadow: 0 5px 15px rgba(0,0,0,0.1);
        }
        
        .character h3 {
            color: #2d3748;
            margin-top: 0;
            border-bottom: 2px solid #667eea;
            padding-bottom: 10px;
        }
        
        .character-info {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 15px;
            margin-bottom: 15px;
        }
        
        .info-item {
            background: white;
            padding: 10px;
            border-radius: 6px;
            border-left: 4px solid #667eea;
        }
        
        .ability-scores {
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            gap: 10px;
        }
        
        .ability {
            text-align: center;
            background: white;
            padding: 10px;
            border-radius: 6px;
            border: 1px solid #e2e8f0;
        }
        
        .ability-name {
            font-size: 12px;
            color: #718096;
            text-transform: uppercase;
            font-weight: 600;
        }
        
        .ability-score {
            font-size: 24px;
            font-weight: bold;
            color: #2d3748;
        }
        
        .loading {
            text-align: center;
            color: #667eea;
            font-style: italic;
        }
        
        .error {
            background: #fed7d7;
            color: #c53030;
            padding: 15px;
            border-radius: 8px;
            margin: 15px 0;
            border-left: 4px solid #e53e3e;
        }
        
        .footer {
            text-align: center;
            margin-top: 30px;
            color: #718096;
            font-size: 14px;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🎲 GLOG v2 Character Generator</h1>
        
        <form id="characterForm">
            <div class="form-group">
                <label for="level">Character Level (1-10):</label>
                <input type="number" id="level" name="level" min="1" max="20" value="1" required>
            </div>
            
            <div class="form-group">
                <label for="count">Number of Characters:</label>
                <input type="number" id="count" name="count" min="1" max="20" value="1" required>
            </div>
            
            <button type="submit" id="generateBtn">Generate Characters</button>
        </form>
        
        <div id="results"></div>
        
        <div class="footer">
            Powered by shared Rust library logic 🦀
        </div>
    </div>

    <script>
        document.getElementById('characterForm').addEventListener('submit', async function(e) {
            e.preventDefault();
            
            const level = document.getElementById('level').value;
            const count = document.getElementById('count').value;
            const generateBtn = document.getElementById('generateBtn');
            const results = document.getElementById('results');
            
            // Validation
            if (level < 1 || level > 20) {
                showError('Level must be between 1 and 20');
                return;
            }
            
            if (count < 1 || count > 20) {
                showError('Number of characters must be between 1 and 20');
                return;
            }
            
            // Show loading state
            generateBtn.disabled = true;
            generateBtn.textContent = 'Generating...';
            results.innerHTML = '<div class="loading">🎲 Rolling dice and creating characters...</div>';
            
            try {
                const response = await fetch('/generate', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({
                        level: parseInt(level),
                        count: parseInt(count)
                    })
                });
                
                const data = await response.json();
                
                if (data.success) {
                    displayCharacters(data.characters);
                } else {
                    showError(data.message);
                }
            } catch (error) {
                showError('Failed to generate characters. Please try again.');
            } finally {
                generateBtn.disabled = false;
                generateBtn.textContent = 'Generate Characters';
            }
        });
        
        function displayCharacters(characters) {
            const results = document.getElementById('results');
            let html = '';
            
            characters.forEach((character, index) => {
                html += `
                    <div class="character">
                        <h3>Character ${index + 1}</h3>
                        <div class="character-info">
                            <div class="info-item">
                                <strong>Level:</strong> ${character.level}
                            </div>
                            <div class="info-item">
                                <strong>Race:</strong> ${character.race}
                            </div>
                            <div class="info-item">
                                <strong>Class:</strong> ${character.class}
                            </div>
                        </div>
                        <h4>Ability Scores:</h4>
                        <div class="ability-scores">
                            <div class="ability">
                                <div class="ability-name">Strength</div>
                                <div class="ability-score">${character.ability_scores.strength}</div>
                            </div>
                            <div class="ability">
                                <div class="ability-name">Dexterity</div>
                                <div class="ability-score">${character.ability_scores.dexterity}</div>
                            </div>
                            <div class="ability">
                                <div class="ability-name">Constitution</div>
                                <div class="ability-score">${character.ability_scores.constitution}</div>
                            </div>
                            <div class="ability">
                                <div class="ability-name">Intelligence</div>
                                <div class="ability-score">${character.ability_scores.intelligence}</div>
                            </div>
                            <div class="ability">
                                <div class="ability-name">Wisdom</div>
                                <div class="ability-score">${character.ability_scores.wisdom}</div>
                            </div>
                            <div class="ability">
                                <div class="ability-name">Charisma</div>
                                <div class="ability-score">${character.ability_scores.charisma}</div>
                            </div>
                        </div>
                    </div>
                `;
            });
            
            document.getElementById('results').innerHTML = html;
        }
        
        function showError(message) {
            document.getElementById('results').innerHTML = `
                <div class="error">
                    <strong>Error:</strong> ${message}
                </div>
            `;
        }
    </script>
</body>
</html>
    "#;
    
    Html(html.to_string())
}

async fn generate_characters(
    State(generator): State<AppState>,
    Json(request): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, StatusCode> {
    // Validate input
    if request.level < 1 || request.level > 20 {
        return Ok(Json(GenerateResponse {
            characters: vec![],
            success: false,
            message: "Level must be between 1 and 20".to_string(),
        }));
    }
    
    if request.count < 1 || request.count > 20 {
        return Ok(Json(GenerateResponse {
            characters: vec![],
            success: false,
            message: "Number of characters must be between 1 and 20".to_string(),
        }));
    }
    
    // Generate characters using shared logic
    match generator.generate_characters(request.level, request.count) {
        Ok(characters) => Ok(Json(GenerateResponse {
            characters,
            success: true,
            message: "Characters generated successfully".to_string(),
        })),
        Err(e) => Ok(Json(GenerateResponse {
            characters: vec![],
            success: false,
            message: e.to_string(),
        })),
    }
}
