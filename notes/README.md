# 2 - Entities and Components

## About Entities and Components
Object Oriented Programming/Design is very common in games. The "class hierachy" would probably look like this:
```
Base Entity
    Monster
        MeleeMob
            OrcWarrior
        ArcherMod
            OrcArcher
```

The problem with this is that it can be restrictive, and before you know you start writing seperate classes for more complicated combinations. For example if there is a Orc that becomes friendly when you completed a quest. 

Entity Component based design tries to eliminate the hierarchy and instead implement a set of "components" that **describe what you want**. An entity is anything and components are just data, grouped by whatever properties a thing, entity has.

For example, you can build the same set of mobs with components for: `Position`, `Renderable`, `Hostile`, `MeleeAI`, `RangedAI` and some sort of CombatStat component. A Orc Warrior would need a position, a renderable, Hostile and MeleeAI. An Archer needs the same just with RangedAI instead. A hybrid could keep both AIs or an additional one. If it becomes friendly you remove the Hostile component, and add a Friendly one.

Components are just like your inheritence tree, but instead of inheriting traits, you compose them by adding them until an entity does what you want. This is called "composition".

The "S" in ECS stands for "Systems". A System is a piece of code that gathers data from an entity/components list and does something with it. For each BaseEntity, call that entitys Draw command.
