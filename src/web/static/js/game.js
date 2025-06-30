// Language translations
const translations = {
    fr: {
        title: '🎲 Jeu 6000 - Humain vs IA 🎲',
        you: '👤 Vous',
        turn: 'Tour',
        roll: 'Lancé',
        rollBtn: 'Lancer les dés',
        bankBtn: 'Sécuriser les points',
        nextBtn: 'Joueur suivant',
        newGameBtn: 'Nouveau Jeu',
        newGameStart: 'Cliquez sur "Nouveau Jeu" pour commencer !',
        yourTurn: 'Lancez les dés pour commencer.',
        aiThinking: 'L\'IA réfléchit... 🤔',
        youRolled: 'Vous avez lancé',
        selectDice: 'Les dés qui rapportent des points sont sélectionnés automatiquement',
        youWon: 'Vous avez gagné',
        points: 'points',
        rulesTitle: '📋 Règles du jeu 6000',
        bust: 'Tous les points sont perdu!',
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
        nextBtn: 'Next Player',
        newGameBtn: 'New Game',
        newGameStart: 'Click "New Game" to start!',
        yourTurn: 'Roll the dice to begin.',
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
let aiPollingInterval = null;
let aiPollingTimeout = null;

function toggleRules() {
    const list = document.getElementById("rules-list");
    const indicator = document.getElementById("toggle-indicator");
    const isVisible = list.style.display === "block";

    list.style.display = isVisible ? "none" : "block";
    indicator.textContent = isVisible ? "[+]" : "[-]";
}

// Affichage générique de l'état du jeu
function renderGameState() {

    const rollBtn = document.getElementById('roll-btn');
    const bankBtn = document.getElementById('bank-btn');
    const nextBtn = document.getElementById('next-player-btn');

    if (!gameState) {
        rollBtn.disabled = true;
        bankBtn.disabled = true;
        nextBtn.disabled = true;
        return;
    }

    const t = translations[currentLanguage];
    const scoresContainer = document.getElementById('game-scores');
    scoresContainer.innerHTML = '';
    gameState.game_state.players.forEach((player, index) => {
        const playerDiv = document.createElement('div');
        playerDiv.className = 'player-score';
        playerDiv.id = `player-${index}-score`;
        let playerName = player.name;
        if (player.is_human) playerName = t.you;
        if (index === gameState.current_player_index) playerDiv.classList.add('current-player');
        playerDiv.innerHTML = `
            <div class="player-name">${playerName}</div>
            <div class="total-score">${player.score}</div>
            <div class="turn-score">${t.turn}: ${player.turn_score ?? 0}</div>
            <div class="roll-score">${t.roll}: ${player.roll_score ?? 0}</div>
        `;
        scoresContainer.appendChild(playerDiv);
    });

    // Dés
    const diceContainer = document.getElementById('dice-container');
    diceContainer.innerHTML = '';
    // Correction : on utilise gameState.rerollable_dice pour colorer en ROUGE (non-scorant), sinon VERT (scorant)
    if (gameState.game_state.dice && Array.isArray(gameState.game_state.dice) && gameState.game_state.dice.length > 0) {
        // Si rerollable_dice n'est pas défini ou vide, tous les dés sont scorants
        const rerollable = Array.isArray(gameState.game_state.rerollable_dice) ? gameState.game_state.rerollable_dice : [];
        gameState.game_state.dice.forEach((die, i) => {
            const dieDiv = document.createElement('div');
            // Correction : si rerollable_dice est vide, tous verts
            if (!gameState.game_state.rerollable_dice || gameState.game_state.rerollable_dice.length === 0) {
                dieDiv.className = 'die scoring-die'; // tous scorants = vert
            } else if (gameState.game_state.rerollable_dice.includes(i)) {
                dieDiv.className = 'die non-scoring'; // non scorant = rouge
            } else {
                dieDiv.className = 'die scoring-die'; // scorant = vert
            }
            dieDiv.textContent = die;
            diceContainer.appendChild(dieDiv);
        });
    }
    // Message générique
    let msg = '';
    const currentPlayer = gameState.game_state.players[gameState.game_state.current_player_index];

    // Nouvelle logique d'activation des boutons
    const turnEndReason = gameState.game_state.turn_end_reason;
    if (turnEndReason) {
        rollBtn.disabled = true;
        bankBtn.disabled = true;
        nextBtn.disabled = false;
    } else {
        nextBtn.disabled = true;
        if (currentPlayer.is_human) {
            rollBtn.disabled = false;
            bankBtn.disabled = false;
            if (currentPlayer.turn_score == 0) {
                bankBtn.disabled = true;
            }
        } else {
            rollBtn.disabled = true;
            bankBtn.disabled = true;
        }
    }

    // Message générique
    if (gameState.ai_decision && gameState.ai_explanation) {
        msg = `${t.aiThinking}: ${gameState.ai_explanation}`;
    } else if (turnEndReason === 'busted') {
        msg = t.bust;
    } else if (turnEndReason === 'banked') {
        msg = `${currentPlayer.name} ${t.banks} ${currentPlayer.turn_score} ${t.points}.`;
    } else if (typeof gameState.current_roll_score === 'number' && gameState.current_roll_score > 0) {
        msg = `${currentPlayer.name} a gagné ${gameState.current_roll_score} points avec ce lancer.`;
    } else {
        msg = `${t.yourTurn}`;
    }
    document.getElementById('game-info').textContent = msg;
}

// Polling générique
async function pollGameStatus() {
    if (!currentGameId) return;
    try {
        const response = await fetch(`/api/game/${currentGameId}/status`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            }
        });
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data = await response.json();
        if (data && data.game_state) {
            gameState = data;
            renderGameState();
        }
        if (aiPollingTimeout) clearTimeout(aiPollingTimeout);
        aiPollingTimeout = setTimeout(pollGameStatus, 3000);
    } catch (error) {
        console.error('Error polling game status:', error);
        const t = translations[currentLanguage];
        document.getElementById('game-info').textContent = `${t.error}: ${error.message}`;
    }
}

// Démarre le polling dès la création du jeu
async function createGame() {
    try {
        const t = translations[currentLanguage];
        document.getElementById('game-info').textContent = t.creating;
        const response = await fetch('/api/game', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ language: currentLanguage })
        });
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data = await response.json();
        if (data.success && data.game_state) {
            gameState = data.game_state;
            currentGameId = data.game_state.id;
            await pollGameStatus();
        } else {
            throw new Error(data.message || 'Failed to create game');
        }
    } catch (error) {
        console.error('Error creating game:', error);
        const t = translations[currentLanguage];
        document.getElementById('game-info').textContent = `${t.error}: ${error.message}`;
    }
}

function setLanguage(lang) {
    currentLanguage = lang;
    document.querySelectorAll('.lang-btn').forEach(btn => btn.classList.remove('active'));
    event.target.classList.add('active');
   
    const t = translations[currentLanguage];
    document.getElementById('main-title').textContent = t.title;
    document.getElementById('roll-text').textContent = t.rollBtn;
    document.getElementById('bank-text').textContent = t.bankBtn;
    document.getElementById('new-game-text').textContent = t.newGameBtn;
    document.getElementById('next-player-text').textContent = t.nextBtn;
    document.getElementById('rules-title').textContent = t.rulesTitle;
    
    // Update game info if no game is active
    if (!gameState) {
        document.getElementById('game-info').textContent = t.newGameStart;
    }
}

// Nouvelle fonction pour passer au joueur suivant via l'API
async function nextPlayer() {
    if (!currentGameId) return;
    try {
        const response = await fetch(`/api/game/${currentGameId}/next`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            }
        });
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data = await response.json();
        if (data.success) {
            gameState = data;
            renderGameState();
        } else {
            throw new Error(data.message || 'Failed to go to next player');
        }
    } catch (error) {
        console.error('Error next player:', error);
        const t = translations[currentLanguage];
        document.getElementById('game-info').textContent = `${t.error}: ${error.message}`;
    }
}

// Appels d'API spécifiques au jeu
async function rollDice() {
    if (!currentGameId) return;
    try {
        const response = await fetch(`/api/game/${currentGameId}/roll`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({}) // Ajoute un body JSON vide pour éviter l'erreur EOF
        });
        let data;
        let rawText = '';
        try {
            data = await response.json();
        } catch (jsonErr) {
            rawText = await response.text();
            console.error('Failed to parse JSON from /roll:', jsonErr);
            console.error('Raw response:', rawText);
            throw new Error(`Invalid JSON from backend: ${rawText}`);
        }
        if (!response.ok) {
            console.error('Backend error response:', data);
            throw new Error(`HTTP error! status: ${response.status} - ${data.message || JSON.stringify(data)}`);
        }
        if (data.success && data.game_state) {
            await pollGameStatus();
        } else {
            console.error('Unexpected /roll response:', data);
            const t = translations[currentLanguage];
            document.getElementById('game-info').textContent = `${t.error}: ${data.message || JSON.stringify(data)}`;
            throw new Error(data.message || JSON.stringify(data) || 'Failed to roll dice');
        }
    } catch (error) {
        console.error('Error rolling dice:', error);
        const t = translations[currentLanguage];
        document.getElementById('game-info').textContent = `${t.error}: ${error.message}`;
    }
}

async function bankPoints() {
    if (!currentGameId) return;
    try {
        const response = await fetch(`/api/game/${currentGameId}/bank`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            }
        });
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data = await response.json();
        if (data.success && data.game_state) {
            await pollGameStatus();
        } else {
            throw new Error(data.message || 'Failed to bank points');
        }
    } catch (error) {
        console.error('Error banking points:', error);
        const t = translations[currentLanguage];
        document.getElementById('game-info').textContent = `${t.error}: ${error.message}`;
    }
}