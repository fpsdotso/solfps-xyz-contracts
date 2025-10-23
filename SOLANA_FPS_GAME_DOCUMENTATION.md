# Solana FPS Game Contract System Documentation

This is a **first-person shooter (FPS) game** built on Solana using the Bolt framework with an Entity Component System (ECS) architecture. The game supports team-based multiplayer matches with lobby functionality.

## Table of Contents

- [Core Components](#core-components)
- [Game Systems](#game-systems)
- [Game Flow](#game-flow)
- [Key Game Features](#key-game-features)
- [Contract Details](#contract-details)

## Core Components

### **Player Component** (`player`)

- **Purpose**: Core player identity and state
- **Key Fields**:
  - `username` (max 32 chars), `team` (0=none, 1=Team A, 2=Team B)
  - `current_game` (PDA reference), `is_alive`, `is_ready`
  - `authority` (player's wallet), `level`, `total_matches_played`

### **Game Component** (`game`)

- **Purpose**: Match/lobby management
- **Key Fields**:
  - `game_state` (0=waiting, 1=active, 2=ended, 3=paused)
  - Team scores, player counts, match duration (5 minutes)
  - Lobby features: `lobby_name`, `created_by`, `ready_players`

### **Health Component** (`health`)

- **Purpose**: Player health and damage tracking
- **Key Fields**: `current_hp` (100), `armor` (50), `is_alive`
- **Special**: `invulnerable_until`, `respawn_timestamp`, damage tracking

### **Weapon Component** (`weapon`)

- **Purpose**: Weapon management and ammo
- **Key Fields**: Primary/secondary weapons, ammo counts, damage values
- **Special**: Reload mechanics, fire rate limiting

### **Position Component** (`position`)

- **Purpose**: 3D movement and physics
- **Key Fields**: `x,y,z` coordinates, `rotation_x`, velocity vectors
- **Special**: Jump mechanics, movement flags

### **PlayerStats Component** (`player_stats`)

- **Purpose**: Match statistics tracking
- **Key Fields**: `kills`, `deaths`, `headshots`, `damage_dealt/taken`
- **Special**: Kill streaks, KDA ratio

---

## Game Systems

### **1. `init_player` - Player Registration**

```rust
// Creates new player account with username
// Validates username (3-32 characters)
// Sets initial player state (not alive, no team, level 1)
```

- **Purpose**: First-time player setup
- **Requirements**: Player must not be already registered
- **Result**: Player can now join games
- **Contract ID**: `DdfNp9wUnuuuTPN1uihkATQUNj9Hzxj6xuZQsBamfzr3`

### **2. `init_game` - Create Game Lobby**

```rust
// Creates new game room with lobby features
// Sets creator as first player, initializes game state
// Configures match settings (5 min duration, 5 players per team)
```

- **Purpose**: Lobby creation for matchmaking
- **Requirements**: Player must be registered and not in another game
- **Result**: Game room ready for players to join
- **Contract ID**: `5ZhDRGff5T4dyvdiDF6Mg729VLQ2EVU4ecqY3ni3PQGo`

### **3. `join_game` - Enter Game Lobby**

```rust
// Auto-assigns team (balances teams)
// Sets player as alive and ready to play
// Updates team player counts
```

- **Purpose**: Matchmaking into existing lobbies
- **Requirements**: Game must be in waiting state, not full
- **Result**: Player added to game, team assigned
- **Contract ID**: `H2ezdHmHvnoQc5T7RCTh8tEEfmvCaSHD7trSAMsVxBAv`

### **4. `leave_game` - Exit Game**

```rust
// Removes player from game, updates team counts
// Auto-ends game if no players left
// Resets player state
```

- **Purpose**: Leave lobby or match
- **Requirements**: Player must be in the game
- **Result**: Player removed, game may end if empty
- **Contract ID**: `8QczrLnLiJeQDk3QjhTyba3KNGXN2Z9fM2Kg2H3hD5RG`

### **5. `set_ready` - Ready Up**

```rust
// Toggles ready state for match start
// Updates ready player count
// Required for game to start
```

- **Purpose**: Lobby ready system
- **Requirements**: Game in waiting state
- **Result**: Player marked as ready/not ready
- **Contract ID**: `ApjerCa4TNZHEPHheK8XSkVdsKdYqq6Yg6fMviECxSqx`

### **6. `start_game` - Begin Match**

```rust
// Validates: lobby owner OR all players ready
// Requires: minimum 2 players
// Changes game state to active, records start time
```

- **Purpose**: Transition from lobby to active match
- **Requirements**: Lobby owner or all players ready + 2+ players
- **Result**: Game becomes active, match begins
- **Contract ID**: `3DUusbymEx6PDZbeHtiZK9vKzGf8M3ePDG7vYegSKBH7`

### **7. `movement` - Player Movement**

```rust
// Processes movement input (WASD, sprint, jump)
// Applies physics (gravity, velocity, collision)
// Updates position and rotation
```

- **Purpose**: Real-time player movement
- **Input**: Movement flags, rotation, sprint state
- **Physics**: Jump mechanics, gravity, speed limits
- **Result**: Updated player position
- **Contract ID**: `5UiPWCCSbCWu5YdkFhe36MK5YL11yLhojrtT7mtGfP7j`

### **8. `shoot` - Weapon Firing**

```rust
// Validates ammo, fire rate, reload state
// Consumes ammo, updates weapon state
// Records shot timestamp for damage calculation
```

- **Purpose**: Weapon firing mechanics
- **Requirements**: Player alive, ammo available, not reloading
- **Result**: Shot fired, ammo consumed
- **Contract ID**: `FDCpMdKzRvkgBnn6BERa9DtqUJY6Fxj3xjsRxFPkhJVh`

### **9. `reload` - Weapon Reloading**

```rust
// Starts reload process with time delay
// Completes reload after timer expires
// Refills ammo from reserves
```

- **Purpose**: Ammo management
- **Requirements**: Weapon not full, has reserve ammo
- **Result**: Ammo refilled after reload time
- **Contract ID**: `8UWDB2GtMLKkF1Xx1a54x8MtbhzyTTX8RuoSXBG3xrsN`

### **10. `apply_damage` - Combat System**

```rust
// Complex damage calculation system:
// - Validates combat (different teams, both alive, same game)
// - Calculates damage (weapon + headshot + distance)
// - Applies armor reduction
// - Updates health, handles death
// - Tracks statistics (kills, damage, streaks)
// - Updates team scores
```

- **Purpose**: Core combat mechanics
- **Requirements**: Valid combat scenario (different teams, alive players)
- **Features**:
  - Headshot multipliers (2x damage)
  - Armor system (absorbs 50% damage)
  - Invulnerability frames
  - Kill tracking and statistics
- **Result**: Damage applied, potential kill, stats updated
- **Contract ID**: `GYpCrumupdHMPpke9fZf5Y66WfX2sYKs1xx22yRhYpoq`

### **11. `respawn` - Player Revival**

```rust
// 5-second death cooldown
// Resets health/armor, spawns at team base
// 3-second invulnerability period
```

- **Purpose**: Death and revival system
- **Requirements**: Player must be dead, cooldown expired
- **Result**: Player revived with full health
- **Contract ID**: `FVZKXQwmxnnKBhiyvBU9psjg3RdmGz87hotvGV18V2un`

### **12. `end_game` - Match Conclusion**

```rust
// Changes game state to ended
// Records end timestamp
// Can be called by any player or system
```

- **Purpose**: Match termination
- **Requirements**: Game must be active
- **Result**: Match officially ended
- **Contract ID**: `9WgqyxzyiCZpDSPSMJ1ef59LD1yrL23N5Yauje6eha54`

### **13. `switch_weapon` - Weapon Switching**

```rust
// Currently placeholder implementation
// Would handle primary/secondary weapon switching
```

- **Purpose**: Weapon selection
- **Status**: Not fully implemented yet
- **Contract ID**: `FpY75Ly4uRawJUfmyKcLps9Z1Kytz6BvypwMyyHJWh6d`

---

## Game Flow

### **Lobby Phase** (Game State 0)

1. Player creates/joins game → `init_game`/`join_game`
2. Players set ready status → `set_ready`
3. Lobby owner or all players ready → `start_game`

### **Active Match** (Game State 1)

1. Players move around → `movement`
2. Combat occurs → `shoot` + `apply_damage`
3. Players reload → `reload`
4. Death/respawn cycle → `respawn`
5. Match ends → `end_game`

### **Match End** (Game State 2)

- Statistics finalized
- Players can leave → `leave_game`
- New matches can be created

---

## Key Game Features

- **Team-based**: 2 teams (A/B) with balanced auto-assignment
- **Lobby System**: Ready-up mechanics, private/public games
- **Combat**: Headshots, armor, invulnerability frames
- **Statistics**: Kills, deaths, damage, kill streaks
- **Physics**: Jump mechanics, gravity, movement
- **Match Management**: 5-minute matches, score tracking
- **Real-time**: Movement and combat systems for live gameplay

---

## Contract Details

### Component IDs

- **Player**: `hfbkKtwhWiTnCySqtkVwoti1AF7Xv3MYRdwxmXA1WeD`
- **Game**: `3f5kd3wkJnmRAWu4jDBfWAh1Fu23wHFz9Fd8cAfr4Wdr`
- **Health**: `8c4sj72LjKi9azxGadFLQ89fvQrgQd3eiP8KfDHa67Rv`
- **Weapon**: `CBbM9mKimGEoMiKoY1bUiAjURTDcqC3k6qr1iXxLfSzk`
- **Position**: `34idayqAQEUBFEQoshs4ZxUDMMaeoGwgGuNA2dN71xFH`
- **PlayerStats**: `3A36U7Y8PqfKN83LdRdPHqTHYdvn3vV1hB8BRcmaBkxK`

### System Contract IDs

- **init_player**: `DdfNp9wUnuuuTPN1uihkATQUNj9Hzxj6xuZQsBamfzr3`
- **init_game**: `5ZhDRGff5T4dyvdiDF6Mg729VLQ2EVU4ecqY3ni3PQGo`
- **join_game**: `H2ezdHmHvnoQc5T7RCTh8tEEfmvCaSHD7trSAMsVxBAv`
- **leave_game**: `8QczrLnLiJeQDk3QjhTyba3KNGXN2Z9fM2Kg2H3hD5RG`
- **set_ready**: `ApjerCa4TNZHEPHheK8XSkVdsKdYqq6Yg6fMviECxSqx`
- **start_game**: `3DUusbymEx6PDZbeHtiZK9vKzGf8M3ePDG7vYegSKBH7`
- **movement**: `5UiPWCCSbCWu5YdkFhe36MK5YL11yLhojrtT7mtGfP7j`
- **shoot**: `FDCpMdKzRvkgBnn6BERa9DtqUJY6Fxj3xjsRxFPkhJVh`
- **reload**: `8UWDB2GtMLKkF1Xx1a54x8MtbhzyTTX8RuoSXBG3xrsN`
- **apply_damage**: `GYpCrumupdHMPpke9fZf5Y66WfX2sYKs1xx22yRhYpoq`
- **respawn**: `FVZKXQwmxnnKBhiyvBU9psjg3RdmGz87hotvGV18V2un`
- **end_game**: `9WgqyxzyiCZpDSPSMJ1ef59LD1yrL23N5Yauje6eha54`
- **switch_weapon**: `FpY75Ly4uRawJUfmyKcLps9Z1Kytz6BvypwMyyHJWh6d`

### Game Mechanics

#### Movement System

- **Base Speed**: 4.0 units/second
- **Sprint Speed**: 7.0 units/second
- **Jump Force**: 10.0 units
- **Gravity**: -9.8 units/second²
- **Update Rate**: 60 FPS (0.016s intervals)

#### Combat System

- **Base Health**: 100 HP
- **Base Armor**: 50 points
- **Headshot Multiplier**: 2x damage
- **Armor Reduction**: 50% damage absorbed
- **Invulnerability**: 3 seconds after respawn
- **Respawn Cooldown**: 5 seconds

#### Weapon System

- **Primary Weapon**: 30 rounds, 90 reserve
- **Secondary Weapon**: 15 rounds, 45 reserve
- **Primary Damage**: 25 HP
- **Secondary Damage**: 50 HP
- **Reload Time**: 2000ms
- **Fire Rate**: Limited by weapon reload time

#### Match Settings

- **Match Duration**: 5 minutes (300 seconds)
- **Max Players per Team**: 5
- **Team Assignment**: Auto-balanced
- **Game Modes**: Team Deathmatch

This is a complete FPS game system with lobby functionality, team-based combat, and comprehensive match management built entirely on Solana using the Bolt framework!
