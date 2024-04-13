# Upgrade Ideas

## Take 1

- Have a resource containing every single update that I picked during my run (very useful for stuff like a UI list of said upgrades)
- Have each game entity (= player, enemy, wall) receive said upgrades when they spawn, and apply new upgrades to new game entities
    - Have a bitmask (or marker components???) saying which entities should get an upgrade (did I make the walls bouncy? did I make myself or the enemies slower)

### Problems
- What if I want to use an upgrade that's regularly for the player on an emeny?
    - I could have a GlobalUpgrades resource (which is what I was thinking of in the above section), but also have a PreloadedUpgrades component that holds the upgrades specific to that entity


```rs
#resource
struct ActiveGlobalUpgrades(Vec<GlobalUpgrade>);

struct GlobalUpgrade {
    relevant_entities: GameEntityBitmap,
    upgrade: &Upgrade,
}

struct Upgrade {
    /// Performs the upgrade on a given entity, for example applying the HP bonus.
    /// Can happen when the entity spawns, or when the upgrade is selected.
    /// (maybe: also when the upgrade stack is recalculated?)
    perform_upgrade: SystemId<In=Entity, Out=()>,
    image, name...
}
```

- This makes the order of upgrades matter. What if I want it to not matter?
    - if the `calculate_upgrades_for_entity` system would take all of the global upgrades, and all of the upgrades on the given entity, sort them by a canonical order, and then reapply them, I think it will work.
        - Won't this cause bugs like in Hearthstone where the HP gets raised each time the upgrades get recalculated?
            - IDK, I can cross that bridge when I get to it. I can have upgrades affect "resource" stats (such as the active HP amount) only if they just got applied, or if I have a +20 HP upgrade, I can first remove 20 HP when recalculating, so that when reapplying that upgrade it will be okay to have it receive the HP again.
- How do I parametrize stuff? For example, if I make a "gain X HP" upgrade, how do I make it possible to create instances of it with a different amount of HP?