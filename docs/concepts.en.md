# Wishes Core Concepts

> [!WARNING]
>
> This document was translated using AI tools and may be inaccurate.
>
> For the most accurate information, please refer to [*concepts.md*](concepts.md) (*Simplified Chinese* version)

> [!NOTE]
>
> This document is intended for users who wish to **understand the internal model of `Wishes`** or **create custom banners**.
>
> No programming background is required — this document describes how `Wishes` works in plain language.
>
> This document was generated with AI tools and reviewed by a human.

The flexibility of `Wishes` comes from a small set of general-purpose abstractions.
Once you understand the concepts in this document, you will be able to **build any banner from scratch** — whether you are simulating the complete gacha mechanics of a specific game, or constructing a custom random system.

## 1. Design Philosophy

The design of `Wishes` revolves around two core ideas:

**First, everything is a tag.**
In most gacha tools, attributes such as "rarity", "type", and "which game it belongs to" tend to be hard-coded into the program. `Wishes` instead abstracts **all descriptive information** into a unified concept called a "tag" — whether it describes card attributes, banner membership, or the grouping of a wish result, everything is expressed the same way. This allows the system to adapt to any game without writing separate code for each one.

**Second, logic and data are separated.**
`Wishes` splits "wishing" into two independent questions:

- **Wish logic** is only responsible for answering: "What type of card should this draw hit?"
- **Decks** are only responsible for answering: "Which cards match this type?"

The two communicate through the intermediate layer of **tags**, and neither knows the other's concrete implementation. The benefits are:

- **Logic is reusable** — the same wish logic can drive multiple banners.
- **Data is extensible** — adding a new game or a new banner usually only requires adding data files, with no code changes.

## 2. Tags: A Unified Way to Describe Things

### 2.1 Regular Tags

A **tag** consists of two parts:

- **Namespace** — describes "what this is about", such as `rarity`, `type`, or `game`.
- **Value** — describes "what it specifically is", such as `5`, `character`, or `genshin`.

When written together, they are separated by a colon, for example `rarity:5`, `type:character`, or `game:genshin`.

Two tags are equal only when **both the namespace and the value are identical**. This means `type:character` and `rarity:character` are two different tags — even though their values are both `character`, they describe different aspects of an object.

`Wishes` includes three common namespaces out of the box:

| Namespace | Purpose | Example |
| :--- | :--- | :--- |
| `game` | Game name | `game:genshin` |
| `type` | Object type | `type:character`, `type:weapon` |
| `rarity` | Rarity | `rarity:5`, `rarity:4` |

Of course, you are free to use other namespaces as well, such as `element:pyro` or `path:destruction`.

### 2.2 Event Tags

**Event tags** are a special category within the tag system: they have **only a value and no namespace**.

Their purpose is not to describe a card's intrinsic attributes, but to describe **"which event group the result of this wish belongs to"**. Common examples include `up` (rate-up), `standard` (standard), `fes` (festival-limited), and `appoint` (appointed).

Why are event tags needed? Consider a typical scenario:

> The same 5-star character card might be just a regular card (`standard`) in **Banner A**, but become a rate-up card (`up`) in **Banner B**.

If "whether it is UP" were hard-coded onto the card, that dynamism would be lost — a single card could not belong to two different event groups at the same time.

Event tags are designed precisely to solve this: they are **not attached to the card itself**, but are **dynamically produced by the wish logic on each draw**. The same card naturally has a different event grouping in different banners.

### 2.3 Tagging

All core objects in `Wishes` — cards, decks, banners, and logic — can **carry a set of tags**. When you describe an object, tags are the "keywords" you attach to it.

For example, a card might simultaneously carry `rarity:5`, `type:character`, and `game:genshin`. The system understands "what this card is" precisely through these tags.

## 3. Cards

**Cards** are the most basic output of the wish flow.

Each card has two core fields:

- **Id** — a globally unique identifier, automatically assigned by the system.
- **Content** — the text displayed for the card, such as a character name or item name. Content may be duplicated.

In addition, the card carries a set of tags that describe its attributes.

> [!NOTE]
>
> A **title** field and an **artwork** (art asset) field for cards are planned for a future version.

## 4. Decks

A **deck** is the definition of a "candidate card pool". Its defining characteristic is:

> A deck **does not store a list of cards**; instead, it **dynamically computes its members through a set of filter rules**.

This means that when cards change (for example, when a new 5-star character is added), the deck will automatically include the matching cards without manual maintenance.

### 4.1 Membership Rules

Membership rules consist of an **ordered** set of conditions, applied in the order they are declared. Conditions fall into the following categories:

**Inclusion conditions** — search globally for matching cards and merge them into the current result:

- **Has all tags** — find cards that have all of the specified tags.
- **Has any tag** — find cards that have any of the specified tags.
- **Specific Ids** — directly add cards with the specified identifiers.

**Filter conditions** — further narrow down the current result:

- **Keep all tags** — within the current result, keep only cards that have all of the specified tags.
- **Keep any tag** — within the current result, keep only cards that have any of the specified tags.

**Exclusion conditions** — remove from the result:

- **Exclude specific Ids** — directly remove cards with the specified identifiers.

#### Execution Order

The **declaration order of conditions matters**. The system processes them in two phases:

1. **Phase 1**: Apply all **inclusion** and **filter** conditions in the order they are declared.
   Order matters here — "Has all tags" merges into the current result, whereas "Keep all tags" intersects with the current result. Swapping their positions can lead to different results.
2. **Phase 2**: Apply all **exclusion** conditions together.
   No matter where "Exclude specific Ids" is placed, it takes effect at the end.

In the end, the deck's members are the set of cards left after all the above conditions have been applied.

### 4.2 Event Groups

**Event groups** are a **further narrowing** on top of the members, describing "which cards are actually included under a given event tag".

For example, a banner might contain "all 5-star characters", some of which are marked as `up` (rate-up) and the rest as `standard`. Event groups are the rules that define "which members belong to `up` and which belong to `standard`".

The condition types for event groups are essentially the same as for membership rules, with the following additional options:

- **All** — take all members.
- **Reference other event groups** — reuse the rules of other event groups (*not yet implemented*).

#### An Important Constraint

> The result of an event group **must be a subset of the deck's members**.

Even if an event group's conditions use global searches such as "Has all tags", the final result will be intersected with the deck's members. This guarantees that no card appears in an event group that does not belong to the deck.

### 4.3 Card Queries

When the system needs to "find candidate cards based on tags", it performs the following steps in order:

1. Compute all members of the deck.
2. If the wish logic provides regular tag conditions, intersect them with all cards in the catalog that match those tags.
3. For each event tag, look up the corresponding event group and filter further; **if an event tag has no corresponding event group, return an empty result immediately**.
4. Return all card identifiers that satisfy the conditions.

This process is handled automatically by the program; you only need to declare the rules when configuring a deck.

## 5. Wish Logic

**Wish logic** is responsible for producing "what type of card this draw should hit". Its output is expressed as **tags**, not as specific cards.

The logic consists of two parts: a **static definition** and a **runtime instance**.

### 5.1 Logic Definition

A **logic definition** is a static blueprint, describing "what this logic is" and "which implementation approach it uses".

`Wishes` supports two kinds of logic implementations:

- **Hard-coded logic** — implemented directly inside the program, with good performance, suited for simulating the complex mechanics of a specific game (such as Genshin's Capturing Radiance).
- **Rule-chain logic** — composed of a series of combinable rules (*rule executors are still under development*).

Currently, `Wishes` includes the following hard-coded logics:

| Name | Description |
| :--- | :--- |
| Genshin Impact Character UP | Includes the Capturing Radiance mechanic |
| Honkai: Star Rail Character UP | Standard large/small pity |

### 5.2 Logic Instance

A **logic instance** is the **runtime state of a logic definition within a specific banner**.

For example, the logic instance for Genshin Impact Character UP records:

- How many draws since the last 5-star
- How many draws since the last 4-star
- Whether the next 5-star is guaranteed (large pity)
- The Capturing Radiance counter

These states are updated with each draw and persisted — this is the foundation for **each banner having its own independent pity counter**.

A logic definition can be **shared across multiple banners**, but each banner's logic instance is **independent**.

### 5.3 Logic Result

After the logic finishes executing, it produces a **logic result**, which contains two groups of information:

- **Regular tag conditions** — describes "what type of card was drawn", for example "5-star character".
- **Event tag conditions** — describes "which event group this draw belongs to", for example `up` or `standard`.

The logic result is the **interface between the logic and the deck**: the logic only cares about which tag combination it produces, and the deck only cares about which cards those tags can match.

## 6. Banners

A **banner** is the user-facing entry point for wishing. It binds together a **deck** and a **logic instance**.

One banner = **one deck** + **one logic instance**.

- Multiple banners can reference **the same deck** — for example, a "Genshin Character UP banner" and a "Genshin Character Standard banner" may share a candidate card pool.
- Multiple banners can also reference **the same logic definition** — for example, several UP banners all use the "Genshin Character UP" logic.
- But each banner's logic instance is **independent** — their pity counters do not affect each other.

This is why you can simulate multiple banners in `Wishes` at the same time, and their pity progress will not interfere with one another.

## 7. What Happens During a Single Wish

When you click the "Wish" button in the interface, `Wishes` performs the following steps in order:

**Step 1: Lock the banner.**
The system obtains the target banner and temporarily locks it, preventing two wish requests from modifying the same state at the same time.

**Step 2: Read the counter.**
The system reads "how many times this banner has been drawn from" from the banner's logic instance.

**Step 3: Generate the random seed.**
The system mixes "user + banner + draw count" into a **deterministic random seed**.
Because the seed is deterministic, the same input always produces the same output — meaning **every draw is reproducible**.

**Step 4: Execute the wish logic.**
The logic instance runs the corresponding logic definition and produces a logic result (containing regular tag conditions and event tag conditions).

**Step 5: Query candidate cards.**
The logic result is handed to the deck, which filters out all matching cards based on the tag conditions.

**Step 6: Randomly select one.**
Among all candidate cards, one is selected with **equal probability**.

**Step 7: Update state.**
"Draw count +1" is written back to the banner's logic instance and persisted.

**Step 8: Return the result.**
The drawn card (along with the event tags for this draw) is returned to the interface for display.

### About Determinism

At present, wishing in `Wishes` is **reproducible**: as long as the user, banner, and draw count are identical, the result is guaranteed to be identical. This design brings two benefits:

- **Auditable** — any draw result can be re-verified.
- **Testable** — developers can precisely verify boundary behavior (such as "the 90th draw must produce a 5-star").

> [!NOTE]
>
> The current random seed in `Wishes` is a simplified implementation, built as `local user Id + banner Id + cumulative draw count`.
>
> In the future, the way the random seed is constructed will be changed so that results differ across devices, while remaining reproducible.

## 8. How the Concepts Relate

In one sentence, here is how these concepts cooperate:

> A **banner** references a **deck** and a **logic instance**. The **logic instance** executes a **logic definition**, producing a **logic result**. The **logic result** is handed to the **deck**, which uses the tag conditions to filter out **cards**. The card that is ultimately selected is the result of this draw.

More specifically:

- **Cards** are static data that carry tags.
- **Decks** are sets of tag-based rules that dynamically compute their members.
- **Logic definitions** are static blueprints for wish mechanics.
- **Logic instances** are the runtime state of a logic definition within a specific banner.
- **Logic results** are the intermediate product through which the logic and the deck communicate.
- **Banners** are the combination of all the above elements, and are what the user interacts with directly.

## 9. Creating a Custom Banner

Suppose you want to simulate a banner from a custom game. The general steps are as follows.

### Step 1: Prepare Cards

Add cards to the cards folder in the data directory, giving each card content and tags.

If you are using the graphical interface, you can create them directly in the *Catalog → Cards* panel.

### Step 2: Create a Deck

Create a new deck and declare:

- **Membership rules** — defines which cards belong to this deck.
- **Event group rules** — defines which cards are included in each event group.
- **Tags** (optional) — tags attached to the deck itself.

### Step 3: Choose or Define Wish Logic

You can use the hard-coded logic built into `Wishes`, or wait for the upcoming custom rule-chain feature.

> [!NOTE]
> The built-in logic currently supports the Genshin Impact and Honkai: Star Rail character UP banners.

### Step 4: Create a Banner

Create a banner, bind the deck and logic instance together, and set an initial state for the logic instance (for example, an initial pity count of 0).

### Step 5: Launch and Validate

After launching `Wishes`, the loader will automatically perform the following checks:

1. **Id uniqueness** — all object Ids must be unique.
2. **Reference integrity** — the deck and logic definition referenced by the banner must exist.
3. **Coverage validation** — the deck must match **all possible output combinations** of the logic. If any output combination finds no candidate cards, loading will fail and report the missing combination.

If validation passes, the banner will appear on the home page and you can begin wishing.

## 10. Further Reading

- [Project README](../README.md)
- [Quick Start](../README.md#quick-start)
