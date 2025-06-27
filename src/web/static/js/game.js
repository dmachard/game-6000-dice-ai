// Language translations
const translations = {
    fr: {
        title: '🎲 Jeu 6000 - Humain vs IA 🎲',
        you: '👤 Vous',
        turn: 'Tour',
        roll: 'Lancé',
        rollBtn: 'Lancer les dés',
        bankBtn: 'Sécuriser les points',
        newGameBtn: 'Nouveau Jeu',
        newGameStart: 'Cliquez sur "Nouveau Jeu" pour commencer !',
        yourTurn: 'À votre tour ! Lancez les dés pour commencer.',
        aiThinking: 'L\'IA réfléchit... 🤔',
        youRolled: 'Vous avez lancé',
        selectDice: 'Les dés qui rapportent des points sont sélectionnés automatiquement',
        youWon: 'Vous avez gagné',
        points: 'points',
        rulesTitle: '📋 Règles du jeu 6000',
        bust: 'fait un bide ! Aucun point marqué.',
        banks: 'sécurise',
        decides: 'décide de continuer...',
        scores: 'marque',
        thisRoll: 'points ce lancer',
        turnTotal: 'total du tour',
        computer: '🤖 Ordinateur',
        aiOpenAI: '🧠 IA (OpenAI)',
        aiClaude: '🧠 IA (Claude)',
        aiOllama: '🧠 IA (Ollama)',
        creating: 'Création du jeu...',
        error: 'Erreur'
    },
    en: {
        title: '🎲 Game 6000 - Human vs AI 🎲',
        you: '👤 You',
        turn: 'Turn',
        roll: 'Roll',
        rollBtn: 'Roll Dice',
        bankBtn: 'Bank Points',
        newGameBtn: 'New Game',
        newGameStart: 'Click "New Game" to start!',
        yourTurn: 'Your turn! Roll the dice to begin.',
        aiThinking: 'AI is thinking... 🤔',
        youRolled: 'You rolled',
        selectDice: 'Scoring dice are automatically selected',
        youWon: 'You won',
        points: 'points',
        rulesTitle: '📋 Game 6000 Rules',
        bust: 'busted! No points scored.',
        banks: 'banks',
        decides: 'decides to continue...',
        scores: 'scores',
        thisRoll: 'points this roll',
        turnTotal: 'turn total',
        computer: '🤖 Computer',
        aiOpenAI: '🧠 AI (OpenAI)',
        aiClaude: '🧠 AI (Claude)',
        aiOllama: '🧠 AI (Ollama)',
        creating: 'Creating game...',
        error: 'Error'
    }
};

let currentLanguage = 'fr';
let gameState = null;
let currentGameId = null;

// Language functions
function setLanguage(lang) {
    currentLanguage = lang;
    document.querySelectorAll('.lang-btn').forEach(btn => btn.classList.remove('active'));
    event.target.classList.add('active');
    updateLanguage();
}

function updateLanguage() {
    const t = translations[currentLanguage];
    document.getElementById('main-title').textContent = t.title;
    document.getElementById('roll-text').textContent = t.rollBtn;
    document.getElementById('bank-text').textContent = t.bankBtn;
    document.getElementById('new-game-text').textContent = t.newGameBtn;
    document.getElementById('rules-title').textContent = t.rulesTitle;
    
    // Update game info if no game is active
    if (!gameState) {
        document.getElementById('game-info').textContent = t.newGameStart;
    }
    
    // Update player names if game is active
    if (gameState) {
        updateScoreDisplay();
    }
}

// API functions
async function createGame() {
    try {
        const t = translations[currentLanguage];
        document.getElementById('game-info').textContent = t.creating;
        disableControls(); // Désactive les boutons pendant la création
        const response = await fetch('/api/game', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                language: currentLanguage
            })
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data = await response.json();
        if (data.success && data.game_state) {
            gameState = data.game_state;
            currentGameId = data.game_state.id;
            updateScoreDisplay();
            updateGameInfo();
            // Active le bouton lancer dès qu'une partie est créée
            document.getElementById('roll-btn').disabled = false;
            document.getElementById('bank-btn').disabled = true;
        } else {
            throw new Error(data.message || 'Failed to create game');
        }
    } catch (error) {
        console.error('Error creating game:', error);
        const t = translations[currentLanguage];
        document.getElementById('game-info').textContent = `${t.error}: ${error.message}`;
    }
}

function updateScoreDisplay() {
    if (!gameState) return;
    const t = translations[currentLanguage];
    const scoresContainer = document.getElementById('game-scores');
    scoresContainer.innerHTML = '';

    gameState.players.forEach((player, index) => {
        const playerDiv = document.createElement('div');
        playerDiv.className = 'player-score';
        playerDiv.id = `player-${index}-score`;
        let playerName = player.name;
        if (player.is_human) {
            playerName = t.you;
        } else if (player.ai_type === 'computer') {
            playerName = t.computer;
        } else if (player.ai_type === 'openai') {
            playerName = t.aiOpenAI;
        } else if (player.ai_type === 'anthropic') {
            playerName = t.aiClaude;
        } else if (player.ai_type === 'ollama') {
            playerName = t.aiOllama;
        }
        if (index === gameState.current_player_index) {
            playerDiv.classList.add('current-player');
        }
        playerDiv.innerHTML = `
            <div class="player-name">${playerName}</div>
            <div class="total-score">${player.score}</div>
            <div class="turn-score">${t.turn}: ${player.turn_score ?? 0}</div>
            <div class="roll-score">${t.roll}: ${player.roll_score ?? 0}</div>
        `;
        scoresContainer.appendChild(playerDiv);
    });

    // Affichage des dés si présents dans gameState
    const diceContainer = document.getElementById('dice-container');
    diceContainer.innerHTML = '';
    if (gameState.dice && Array.isArray(gameState.dice)) {
        gameState.dice.forEach((die, i) => {
            const dieDiv = document.createElement('div');
            // Dés non scorants (dans rerollable_dice) → rouge, scorants → vert
            if (gameState.rerollable_dice && gameState.rerollable_dice.includes(i)) {
                dieDiv.className = 'die scoring-die'; // non scorant = rouge
            } else {
                dieDiv.className = 'die rerollable-die'; // scorant = vert
            }
            dieDiv.textContent = die;
            diceContainer.appendChild(dieDiv);
        });
    }
    // Suppression de l'affichage du score du lancé sous les dés
    const rollScoreDiv = document.getElementById('roll-score-info');
    if (rollScoreDiv) {
        rollScoreDiv.remove();
    }
}

function updateGameInfo() {
    if (!gameState) return;
    const t = translations[currentLanguage];
    const currentPlayer = gameState.players[gameState.current_player_index];
    // Message spécial si bust
    if (gameState.busted) {
        document.getElementById('game-info').textContent = t.bust + ' → ' + t.yourTurn;
        // Affiche le bouton "Joueur suivant"
        showNextPlayerButton();
        return;
    }
    // Cas spécial : début de partie, aucun dé lancé
    if (!gameState.dice || gameState.dice.length === 0) {
        if (currentPlayer.is_human) {
            document.getElementById('game-info').textContent = t.yourTurn;
        } else {
            document.getElementById('game-info').textContent = `${currentPlayer.name} ${t.aiThinking}`;
        }
        return;
    }
    // Si on ne peut plus continuer (plus de dés à relancer OU can_continue faux)
    if (!gameState.can_continue || (gameState.rerollable_dice && gameState.rerollable_dice.length === 0)) {
        document.getElementById('game-info').textContent = 'Tour terminé, sécurisez vos points !';
        document.getElementById('roll-btn').disabled = true;
        return;
    }
    // Message après un lancer réussi
    if (currentPlayer.is_human && typeof gameState.current_roll_score === 'number' && gameState.current_roll_score > 0) {
        document.getElementById('game-info').textContent = `Vous avez gagné ${gameState.current_roll_score} points avec ce lancer.`;
        return;
    }
    if (currentPlayer.is_human) {
        document.getElementById('game-info').textContent = t.yourTurn;
    } else {
        document.getElementById('game-info').textContent = `${currentPlayer.name} ${t.aiThinking}`;
    }
}

function showNextPlayerButton() {
    let btn = document.getElementById('next-player-btn');
    if (!btn) {
        btn = document.createElement('button');
        btn.id = 'next-player-btn';
        btn.className = 'btn-secondary';
        btn.textContent = 'Joueur suivant';
        btn.onclick = async function() {
            btn.disabled = true;
            // On relance un roll pour le joueur suivant (ou appelle une route dédiée si besoin)
            await rollDice();
            btn.remove();
        };
        document.getElementById('controls').appendChild(btn);
    } else {
        btn.disabled = false;
    }
}

function enableControls() {
    if (!gameState) return;
    const currentPlayer = gameState.players[gameState.current_player_index];
    // Par défaut, tout est désactivé
    document.getElementById('roll-btn').disabled = true;
    document.getElementById('bank-btn').disabled = true;

    // Si c'est au joueur humain de jouer
    if (currentPlayer.is_human) {
        // Si on peut continuer, on active les deux boutons
        if (gameState.can_continue) {
            document.getElementById('roll-btn').disabled = false;
            document.getElementById('bank-btn').disabled = false;
        } else {
            // Début de tour ou après un bust, tout reste désactivé
            document.getElementById('roll-btn').disabled = true;
            document.getElementById('bank-btn').disabled = true;
        }
    }
}

// Au chargement, le bouton sécuriser doit être désactivé
function disableControls() {
    document.getElementById('roll-btn').disabled = true;
    document.getElementById('bank-btn').disabled = true;
}

// Game functions
async function newGame() {
    gameState = null;
    currentGameId = null;
    document.getElementById('game-scores').innerHTML = '';
    document.getElementById('dice-container').innerHTML = '';
    disableControls();
    
    await createGame();
}

async function rollDice() {
    if (!gameState || !currentGameId) return;
    try {
        const response = await fetch(`/api/game/${currentGameId}/roll`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ player_id: null }) // Adapt if player_id is needed
        });
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data = await response.json();
        if (data && data.dice) {
            // Update game state and UI
            gameState.dice = data.dice;
            gameState.rerollable_dice = data.rerollable_dice;
            gameState.current_roll_score = data.roll_score;
            gameState.current_turn_score = data.turn_score;
            gameState.busted = data.busted;
            gameState.can_continue = data.can_continue;
            // Met à jour le score du joueur courant dans gameState.players
            if (gameState.players && gameState.players[gameState.current_player_index]) {
                gameState.players[gameState.current_player_index].turn_score = data.turn_score;
                gameState.players[gameState.current_player_index].roll_score = data.roll_score;
            }
            updateScoreDisplay();
            updateGameInfo();
            enableControls();
        } else if (data && data.busted) {
            // Handle bust
            gameState.busted = true;
            gameState.current_turn_score = 0;
            gameState.current_roll_score = 0;
            if (gameState.players && gameState.players[gameState.current_player_index]) {
                gameState.players[gameState.current_player_index].turn_score = 0;
                gameState.players[gameState.current_player_index].roll_score = 0;
            }
            updateScoreDisplay();
            updateGameInfo();
            enableControls();
        }
    } catch (error) {
        console.error('Error rolling dice:', error);
        const t = translations[currentLanguage];
        document.getElementById('game-info').textContent = `${t.error}: ${error.message}`;
    }
}

async function bankPoints() {
    if (!gameState || !currentGameId) return;
    
    try {
        // Placeholder for bank points API call
        console.log('Banking points...');
        // TODO: Implement bank points API call
        
    } catch (error) {
        console.error('Error banking points:', error);
    }
}

// Initialize the game on page load
document.addEventListener('DOMContentLoaded', function() {
    updateLanguage();
    disableControls();
});