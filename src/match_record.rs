use serde::{Deserialize, Deserializer};

use crate::types::Millisec;

fn deserialize_as_i32<'de, D: Deserializer<'de>>(d: D) -> Result<Option<i32>, D::Error> {
    Option::<f64>::deserialize(d).map(|v| v.map(|f| f as i32))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchRecord {
    pub id: u64,
    pub tag: Option<String>,
    pub date: u64,
    pub rank: Rank,
    pub seed: Option<Seed>,
    #[serde(rename = "type")]
    pub match_type: u8,
    pub result: Option<MatchResult>,
    pub season: u8,
    pub changes: Vec<Change>,
    pub decayed: bool,
    pub players: Vec<Player>,
    pub beginner: bool,
    pub category: Option<String>,
    pub seed_type: Option<String>,
    pub bot_source: Option<String>,
    pub forfeited: bool,
    pub timelines: Vec<Timeline>,
    pub spectators: serde_json::Value,
    pub bastion_type: Option<String>,
    pub completions: Vec<Completion>,
    pub replay_exist: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rank {
    pub season: Option<u32>,
    pub all_time: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Seed {
    pub id: Option<u64>,
    pub nether: Option<String>,
    pub end_towers: Option<String>,
    pub overworld: Option<String>,
    pub variations: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct MatchResult {
    pub time: Millisec,
    pub uuid: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    pub uuid: String,
    #[serde(deserialize_with = "deserialize_as_i32")]
    pub change: Option<i32>,
    #[serde(deserialize_with = "deserialize_as_i32")]
    pub elo_rate: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub uuid: String,
    pub country: Option<String>,
    pub elo_rank: Option<u32>,
    pub elo_rate: Option<u32>,
    pub nickname: String,
    pub role_type: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Timeline {
    pub time: Millisec,
    #[serde(rename = "type")]
    pub timeline_type: TimelineType,
    pub uuid: String,
}

#[derive(Debug)]
pub struct PlayerTimeline {
    pub time: Millisec,
    pub timeline_type: TimelineType,
}

#[derive(Debug)]
pub struct Duel {
    pub record: MatchRecord,
    pub timelines: (Vec<PlayerTimeline>, Vec<PlayerTimeline>),
}

#[derive(Debug, Deserialize)]
pub struct Completion {
    pub time: Millisec,
    pub uuid: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum TimelineType {
    // Adventure
    #[serde(rename = "adventure.adventuring_time")]
    AdventureAdventuringTime,
    #[serde(rename = "adventure.arbalistic")]
    AdventureArbalistic,
    #[serde(rename = "adventure.bullseye")]
    AdventureBullseye,
    #[serde(rename = "adventure.hero_of_the_village")]
    AdventureHeroOfTheVillage,
    #[serde(rename = "adventure.honey_block_slide")]
    AdventureHoneyBlockSlide,
    #[serde(rename = "adventure.kill_a_mob")]
    AdventureKillAMob,
    #[serde(rename = "adventure.kill_all_mobs")]
    AdventureKillAllMobs,
    #[serde(rename = "adventure.ol_betsy")]
    AdventureOlBetsy,
    #[serde(rename = "adventure.root")]
    AdventureRoot,
    #[serde(rename = "adventure.shoot_arrow")]
    AdventureShootArrow,
    #[serde(rename = "adventure.sleep_in_bed")]
    AdventureSleepInBed,
    #[serde(rename = "adventure.sniper_duel")]
    AdventureSniperDuel,
    #[serde(rename = "adventure.summon_iron_golem")]
    AdventureSummonIronGolem,
    #[serde(rename = "adventure.throw_trident")]
    AdventureThrowTrident,
    #[serde(rename = "adventure.totem_of_undying")]
    AdventureTotemOfUndying,
    #[serde(rename = "adventure.trade")]
    AdventureTrade,
    #[serde(rename = "adventure.two_birds_one_arrow")]
    AdventureTwoBirdsOneArrow,
    #[serde(rename = "adventure.very_very_frightening")]
    AdventureVeryVeryFrightening,
    #[serde(rename = "adventure.voluntary_exile")]
    AdventureVoluntaryExile,
    #[serde(rename = "adventure.whos_the_pillager_now")]
    AdventureWhosThePillagerNow,

    // End
    #[serde(rename = "end.dragon_breath")]
    EndDragonBreath,
    #[serde(rename = "end.dragon_egg")]
    EndDragonEgg,
    #[serde(rename = "end.elytra")]
    EndElytra,
    #[serde(rename = "end.enter_end_gateway")]
    EndEnterEndGateway,
    #[serde(rename = "end.find_end_city")]
    EndFindEndCity,
    #[serde(rename = "end.kill_dragon")]
    EndKillDragon,
    #[serde(rename = "end.levitate")]
    EndLevitate,
    #[serde(rename = "end.respawn_dragon")]
    EndRespawnDragon,
    #[serde(rename = "end.root")]
    EndRoot,

    // Husbandry
    #[serde(rename = "husbandry.balanced_diet")]
    HusbandryBalancedDiet,
    #[serde(rename = "husbandry.bred_all_animals")]
    HusbandryBredAllAnimals,
    #[serde(rename = "husbandry.breed_an_animal")]
    HusbandryBreedAnAnimal,
    #[serde(rename = "husbandry.complete_catalogue")]
    HusbandryCompleteCatalogue,
    #[serde(rename = "husbandry.fishy_business")]
    HusbandryFishyBusiness,
    #[serde(rename = "husbandry.obtain_netherite_hoe")]
    HusbandryObtainNetheriteHoe,
    #[serde(rename = "husbandry.plant_seed")]
    HusbandryPlantSeed,
    #[serde(rename = "husbandry.root")]
    HusbandryRoot,
    #[serde(rename = "husbandry.safely_harvest_honey")]
    HusbandrySafelyHarvestHoney,
    #[serde(rename = "husbandry.silk_touch_nest")]
    HusbandrySilkTouchNest,
    #[serde(rename = "husbandry.tactical_fishing")]
    HusbandryTacticalFishing,
    #[serde(rename = "husbandry.tame_an_animal")]
    HusbandryTameAnAnimal,

    // Nether
    #[serde(rename = "nether.all_effects")]
    NetherAllEffects,
    #[serde(rename = "nether.all_potions")]
    NetherAllPotions,
    #[serde(rename = "nether.brew_potion")]
    NetherBrewPotion,
    #[serde(rename = "nether.charge_respawn_anchor")]
    NetherChargeRespawnAnchor,
    #[serde(rename = "nether.create_beacon")]
    NetherCreateBeacon,
    #[serde(rename = "nether.create_full_beacon")]
    NetherCreateFullBeacon,
    #[serde(rename = "nether.distract_piglin")]
    NetherDistractPiglin,
    #[serde(rename = "nether.explore_nether")]
    NetherExploreNether,
    #[serde(rename = "nether.fast_travel")]
    NetherFastTravel,
    #[serde(rename = "nether.find_bastion")]
    NetherFindBastion,
    #[serde(rename = "nether.find_fortress")]
    NetherFindFortress,
    #[serde(rename = "nether.get_wither_skull")]
    NetherGetWitherSkull,
    #[serde(rename = "nether.loot_bastion")]
    NetherLootBastion,
    #[serde(rename = "nether.netherite_armor")]
    NetherNetheriteArmor,
    #[serde(rename = "nether.obtain_ancient_debris")]
    NetherObtainAncientDebris,
    #[serde(rename = "nether.obtain_blaze_rod")]
    NetherObtainBlazeRod,
    #[serde(rename = "nether.obtain_crying_obsidian")]
    NetherObtainCryingObsidian,
    #[serde(rename = "nether.return_to_sender")]
    NetherReturnToSender,
    #[serde(rename = "nether.ride_strider")]
    NetherRideStrider,
    #[serde(rename = "nether.root")]
    NetherRoot,
    #[serde(rename = "nether.summon_wither")]
    NetherSummonWither,
    #[serde(rename = "nether.uneasy_alliance")]
    NetherUneasyAlliance,
    #[serde(rename = "nether.use_lodestone")]
    NetherUseLodestone,

    // Project Elo
    #[serde(rename = "projectelo.timeline.blind_travel")]
    ProjecteloBlindTravel,
    #[serde(rename = "projectelo.timeline.complete")]
    ProjecteloComplete,
    #[serde(rename = "projectelo.timeline.death")]
    ProjecteloDeath,
    #[serde(rename = "projectelo.timeline.death_spawnpoint")]
    ProjecteloDeathSpawnpoint,
    #[serde(rename = "projectelo.timeline.dragon_death")]
    ProjecteloDragonDeath,
    #[serde(rename = "projectelo.timeline.forfeit")]
    ProjecteloForfeit,
    #[serde(rename = "projectelo.timeline.reset")]
    ProjecteloReset,

    // Story
    #[serde(rename = "story.cure_zombie_villager")]
    StoryCureZombieVillager,
    #[serde(rename = "story.deflect_arrow")]
    StoryDeflectArrow,
    #[serde(rename = "story.enchant_item")]
    StoryEnchantItem,
    #[serde(rename = "story.enter_the_end")] // should be same with end.root, same as nether.root
    StoryEnterTheEnd,
    #[serde(rename = "story.enter_the_nether")]
    StoryEnterTheNether,
    #[serde(rename = "story.follow_ender_eye")]
    StoryFollowEnderEye,
    #[serde(rename = "story.form_obsidian")]
    StoryFormObsidian,
    #[serde(rename = "story.iron_tools")]
    StoryIronTools,
    #[serde(rename = "story.lava_bucket")]
    StoryLavaBucket,
    #[serde(rename = "story.mine_diamond")]
    StoryMineDiamond,
    #[serde(rename = "story.mine_stone")]
    StoryMineStone,
    #[serde(rename = "story.obtain_armor")]
    StoryObtainArmor,
    #[serde(rename = "story.root")]
    StoryRoot,
    #[serde(rename = "story.shiny_gear")]
    StoryShinyGear,
    #[serde(rename = "story.smelt_iron")]
    StorySmeltIron,
    #[serde(rename = "story.upgrade_tools")]
    StoryUpgradeTools,
}
