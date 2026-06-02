# Packet Scheme
All event data is transmitted using a binary `Packet` structure.

## Binary Packet Structure
| Body Length | Serialized JSON Body |
| ----------- | -------------------- |
| 4 bytes     | N bytes              |

## Serialized JSON Body Structure
| Field | Type   | Description               |
| ----- | ------ | ------------------------- |
| type  | string | Name of the `EventType`   |
| data  | any    | The `EventType` structure |


# Events
This section describes events dispatched by the server.

| EventType         | Description                                                 |
| ----------------- | ----------------------------------------------------------- |
| Heartbeat         | Heartbeat dispatched every second.                          |
| Error             | Dispatched when handling an event.                          |
| OnBattleBegin     | Dispatched when battle begins.                              |
| OnSetBattleLineup | Dispatched when setting up battle lineup avatars.           |
| OnDamage          | Dispatched when any avatar inflicts any instance of damage. |
| OnTurnBegin       | Dispatched when any entity's turn begins.                   |
| OnTurnEnd         | Dispatched when any entity's turn ends.                     |
| OnKill            | Dispatched when any avatar kills an enemy.                  |
| OnUseSkill        | Dispatched when any avatar uses any skill.                  |
| OnStatChange      | Dispatched when an entity's stat changes.                  |
| OnBattleEnd       | Dispatched when battle ends.                                |

##  Heartbeat
Heartbeat dispatched every second. Clients can handle this event to determine whether the connection is alive.

## Error
Dispatched when handling an event.

### Structure

| Field | Type   | Description                  |
| ----- | ------ | ---------------------------- |
| msg   | string | Human-readable error message |

---

## OnBattleBegin
Dispatched when battle begins.

## OnSetBattleLineup
Dispatched when setting up battle lineup avatars.

### Structure

| Field   | Type                | Description                              |
| ------- | ------------------- | ---------------------------------------- |
| avatars | [Avatar](#avatar)[] | List of all avatars in the battle lineup |

---

## OnDamage
Dispatched when any avatar inflicts any instance of damage.

### Structure

| Field    | Type              | Description                 |
| -------- | ----------------- | --------------------------- |
| attacker | [Avatar](#avatar) | The attacking avatar        |
| damage   | float             | The damage inflicted on hit |

---

## OnTurnBegin
Dispatched when any entity's turn begins.

### Structure

| Field        | Type  | Description          |
| ------------ | ----- | -------------------- |
| action_value | float | Current action value |

---

## OnTurnEnd
Dispatched when any entity's turn ends.

### Structure

| Field          | Type                | Description                              |
| -------------- | ------------------- | ---------------------------------------- |
| avatars        | [Avatar](#avatar)[] | List of all avatars in the battle lineup |
| avatars_damage | float[]             | Total damage dealt by each avatar        |
| total_damage   | float               | Total damage dealt during this turn      |
| action_value   | float               | Action value of this turn                |

---

## OnKill
Dispatched when any avatar kills an enemy.

### Structure

| Field    | Type              | Description       |
| -------- | ----------------- | ----------------- |
| attacker | [Avatar](#avatar) | The killer avatar |


## OnUseSkill
Dispatched when any avatar uses any skill.

### Structure

| Field  | Type              | Description                |
| ------ | ----------------- | -------------------------- |
| avatar | [Avatar](#avatar) | The avatar using the skill |
| skill  | [Skill](#skill)   | The skill that was used    |

---

## OnStatChange
Dispatched when an entity's stat changes.

### Structure

| Field    | Type               | Description                         |
| -------- | ------------------ | ----------------------------------- |
| entity   | [Entity](#entity)  | The entity whose stat changed       |
| property | [Property](#property) | The updated stat payload         |

---

## OnBattleEnd
Dispatched when battle ends. Final summary of the battle.

### Structure

| Field        | Type                    | Description                              |
| ------------ | ----------------------- | ---------------------------------------- |
| avatars      | [Avatar](#avatar)[]     | List of all avatars in the battle lineup |
| turn_history | [TurnInfo](#turninfo)[] | History of all turns                     |
| turn_count   | integer                 | Total number of turns                    |
| total_damage | float                   | Total damage dealt throughout the battle |
| action_value | float                   | Final action value                       |

---

# Object Definitions

## Avatar

| Field | Type    | Description        |
| ----- | ------- | ------------------ |
| id    | integer | Avatar ID          |
| name  | string  | Name of the avatar |

## Skill


| Field | Type   | Description       |
| ----- | ------ | ----------------- |
| name  | string | Name of the skill |
| type  | string | Type of skill     |

## Entity

| Field | Type    | Description     |
| ----- | ------- | --------------- |
| uid   | integer | Entity ID       |
| team  | string  | Player or enemy |

## Property

| Field | Type   | Description              |
| ----- | ------ | ------------------------ |
| value | float  | The current stat value   |
| type  | string | The stat identifier name |

## TurnInfo

| Field               | Type    | Description                         |
| ------------------- | ------- | ----------------------------------- |
| action_value        | float   | Action value of this turn           |
| avatars_turn_damage | float[] | Total damage dealt by each avatar   |
| total_damage        | float   | Total damage dealt during this turn |