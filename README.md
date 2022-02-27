

## Tasks

state holds queues of tasks, each queue is dequeued if there is an npc of the right kind with no task or one with a lower priority

### Scheduler

assigns tasks

### Example Tasks

#### goto cursor
* priority middle
* iterator over list of positions
* change position step by step

#### mine
* priority middle
* go to position
* play animation for some time
* modify map accordingly

#### worker idle
* prio lowest
* repeat
    * go to random spot with walk speed
        * if encountering enemies: alarm and flee

#### patrol idle
* prio lowest
* repeat
    * go to random spot with walk speed
        * if encountering enemies: alarm

#### alarm
* priority high
* try to overwrite task for all soldiers to 'attack at' (if priority allows)

#### attack at
* priority high
* go to position
* repeat
    * if find enemies in attack range: attack
    * elseif find enemies in a vision range: go to
    * else end task

#### flee
* triggered when hp falls under threshold
* priority highest
* repeat
    * try to increase distance to closest enemies positon




same calc for bg and fg

| `last_color` | `current_color` | set color | update `last_color` |
| ------------ | --------------- | --------- | ------------------- |
| 1            | 2               | 2         | y                   |
| 1            | 1               | None      | n                   |
| 1            | None            | None      | n                   |

```mermaid
flowchart TB
    main[[main]]
    Config[Config]
    TileConfig[TileConfig]
    KeyConfig[KeyConfig]
    Map[Map]
    Screen[Screen]
    State[State]
    Renderer[Renderer]

    subgraph Controller
        direction TB
        EventHandler[/KeyEventHandler/]
        KeyEventHandler[/KeyEventHandler/]
        ResizeEventHandler[/ResizeEventHandler/]
        ElapseEventHandler[/ElapseEventHandler/]

        EventHandler --> KeyEventHandler
        EventHandler --> ResizeEventHandler
        EventHandler --> ElapseEventHandler
    end

    main --> Controller

    Controller --> Renderer
    Controller --> State
    Controller --> Config

    Config --> TileConfig
    Config --> KeyConfig

    Renderer --> Screen

    State -.-> |cols, rows| Screen
    State --> Map

    Map -.-> TileConfig
```

```
IIIII
IIIII

IIIIIII
IIIIIII
IIIIIII
```

cursor size is 7 x 17, so what are th best sizes?

```
7*0, 17*0 --- 0
7*2, 17*1 --- -3
7*3, 17*1 --- 4
7*5, 17*2 --- 1
7*7, 17*3 --- -2
7*10, 17*4 --- 2
7*12, 17*5 --- -1
7*14, 17*6 --- -4
7*15, 17*6 --- 3
7*17, 17*7 --- 0
7*19, 17*8 --- -3
```

521 x 420
1.24047619048

```
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
[x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x][x]
```

529 x 642
1.21361058601

```
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
[][][][][][][][][][][][[][][][][][][][][][][][][][]][][][][][][][][][][][][]
```
