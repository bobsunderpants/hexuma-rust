//! Game state, world model and English parser for Hexuma.
//!
//! The original game is a German graphical text adventure.  This module is a
//! faithful English re-implementation of its world and puzzle chain, using the
//! original room numbers (which also drive the picture lookup in BILDDAT.SEQ).

use std::collections::HashSet;

pub const MAX_SCORE: i32 = 1617;

pub struct Game {
    pub room: u16,
    pub flags: HashSet<String>,
    pub inv: Vec<&'static str>,
    pub score: i32,
    pub log: Vec<String>,
    pub ended: bool,
    /// Coloured field the player currently stands on in the stone circle.
    pub field: Option<&'static str>,
    /// Number of labyrinth chambers solved with Pit's riddles.
    pub lab_step: u8,
    pub chapter: u8,
    pub moves: u32,
}

pub fn item_name(id: &str) -> &'static str {
    match id {
        "sword" => "samurai sword",
        "blade" => "sword blade",
        "handle" => "wooden handle",
        "shaft" => "wooden shaft",
        "tip" => "brass tip",
        "cue" => "billiard cue",
        "cue_shaft" => "shaft with handle",
        "hook" => "iron hook",
        "grapple" => "grappling hook",
        "boathook" => "boat hook",
        "wrecking" => "wrecking ball",
        "rope" => "rope",
        "candle" => "candle",
        "lighter" => "lighter",
        "jar" => "jam jar",
        "wax" => "tube of floor wax",
        "sponge" => "sponge",
        "toothbrush" => "toothbrush",
        "coin" => "coin",
        "key" => "small key",
        "car_key" => "ignition key",
        "marble" => "marble ball",
        "pickaxe" => "pickaxe",
        "envelope" => "envelope",
        "letter" => "letter",
        "diary" => "diary",
        "note" => "note",
        "paper" => "sheet of paper",
        "bottle" => "bottle of 1922 petroleum",
        "club" => "wooden club",
        "flag" => "flag",
        "boots" => "boots",
        "coat" => "fur coat",
        "goggles" => "snow goggles",
        "torch" => "torch",
        "snow" => "handful of snow",
        "icekey" => "iron key",
        "rammer" => "powder rammer",
        "powder" => "gunpowder",
        "coconut" => "coconut",
        "stone" => "stone",
        "bottle_maggi" => "bottle of Maggi seasoning",
        "cork" => "crown cork",
        "cord" => "envelope cord",
        "mushroom" => "mushroom",
        "champignon" => "champignon",
        "salt" => "sea salt",
        "rosemary" => "rosemary",
        "oregano" => "oregano",
        "pepper" => "pepper",
        "elixir" => "flask of elixir",
        "eye" => "Eye of Kal",
        "amulet" => "amulet",
        "ball" => "steel ball",
        "speaker" => "loudspeaker",
        "microphone" => "microphone",
        "oxygen" => "oxygen gear",
        "wrench" => "adjustable wrench",
        "scale" => "reflective scale",
        "root" => "root vine",
        "wood" => "piece of wood",
        "spider" => "monster spider",
        "bananas" => "bunch of bananas",
        "meat" => "piece of meat",
        "dung" => "dung clump",
        "leaves" => "handful of leaves",
        "tin" => "breakfast tin",
        "hammer" => "hammer",
        "crowbar" => "crowbar",
        "passkey" => "passkey",
        "fuse" => "fuse",
        "plug" => "electric plug",
        "tongs" => "pair of tongs",
        "card" => "code card",
        "card_coded" => "coded code card",
        "quicksand" => "handful of quicksand",
        "bucket" => "bucket",
        "gear" => "oxygen gear",
        "sigma" => "sigma cell",
        "hood" => "fur hood",
        "gloves" => "gloves",
        "twig" => "tree twig",
        "pole" => "wooden pole",
        "sail" => "sailcloth",
        "bible" => "children's bible",
        "soot" => "handful of soot",
        "chest" => "sea chest",
        "fuel" => "lighter fuel",
        "sugar" => "sugar cube",
        "fly" => "dead fly",
        "cube" => "stone cube",
        _ => "object",
    }
}

fn normalize_dir(w: &str) -> Option<&'static str> {
    match w {
        "n" | "north" | "norden" => Some("north"),
        "s" | "south" | "sueden" | "süden" => Some("south"),
        "e" | "east" | "osten" => Some("east"),
        "w" | "west" | "westen" => Some("west"),
        "u" | "up" | "above" | "hoch" | "hinauf" => Some("up"),
        "d" | "down" | "below" | "runter" | "hinunter" => Some("down"),
        "ne" | "northeast" => Some("northeast"),
        "nw" | "northwest" => Some("northwest"),
        "se" | "southeast" => Some("southeast"),
        "sw" | "southwest" => Some("southwest"),
        "in" | "enter" | "inside" | "hinein" => Some("in"),
        "out" | "exit" | "outside" | "hinaus" => Some("out"),
        _ => None,
    }
}

impl Game {
    pub fn new() -> Game {
        let mut g = Game {
            room: 1,
            flags: HashSet::new(),
            inv: Vec::new(),
            score: 0,
            log: Vec::new(),
            ended: false,
            field: None,
            lab_step: 0,
            chapter: 1,
            moves: 0,
        };
        g.print("HEXUMA: THE EYE OF KAL");
        g.print("An English remake of the 1992 adventure by Weltenschmiede.");
        g.print("");
        g.print("Type HELP for commands, JOURNAL for your notes, or HINT if you are stuck.");
        g.enter(1);
        g
    }

    pub fn print(&mut self, msg: &str) {
        for line in wrap(msg, 58) {
            self.log.push(line);
        }
        if self.log.len() > 20000 {
            let cut = self.log.len() - 20000;
            self.log.drain(0..cut);
        }
    }

    pub fn has(&self, item: &str) -> bool {
        self.inv.iter().any(|i| *i == item)
    }

    pub fn flag(&self, f: &str) -> bool {
        self.flags.contains(f)
    }

    pub fn set(&mut self, f: &str) {
        self.flags.insert(f.to_string());
    }

    pub fn unset(&mut self, f: &str) {
        self.flags.remove(f);
    }

    /// Number of gem splinters the player currently owns.
    pub fn shards(&self) -> i32 {
        (1..=6).filter(|i| self.flag(&format!("shard{i}"))).count() as i32
    }

    pub fn has_shard(&self, n: i32) -> bool {
        self.flag(&format!("shard{n}"))
    }

    pub fn give_shard(&mut self, n: i32) {
        self.set(&format!("shard{n}"));
    }

    pub fn lose_shard(&mut self, n: i32) {
        self.unset(&format!("shard{n}"));
    }

    pub fn add(&mut self, item: &'static str) {
        if !self.has(item) {
            self.inv.push(item);
        }
    }

    pub fn remove(&mut self, item: &str) -> bool {
        if let Some(pos) = self.inv.iter().position(|i| *i == item) {
            self.inv.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn award(&mut self, points: i32) {
        self.score += points;
        if points > 0 {
            self.print(&format!("[{points} points]"));
        }
    }

    // ------------------------------------------------------------------
    // Room meta data
    // ------------------------------------------------------------------

    pub fn room_name(&self, id: u16) -> &'static str {
        match id {
            1 => "Entrance Hall",
            2 => "Library",
            3 => "Drawing Room",
            4 => "Billiard Room",
            5 => "Fireplace",
            6 => "Kitchen Door",
            7 => "Bedroom",
            8 => "Bathroom",
            9 => "Upstairs Hall",
            10 => "Kitchen",
            11 => "Nursery",
            12 => "Attic",
            13 => "Attic Storeroom",
            14 => "Wine Cellar",
            15 => "Junk Room",
            16 => "Cellar",
            24 => "The Black Pit",
            26 => "The Stone Circle",
            27 => "Ice Niche",
            28 => "Ice Slope",
            29 => "Steep Slope",
            30 => "Frozen Ledge",
            31 => "Ice Cave Mouth",
            32 => "Monolith Plateau",
            33 => "Crater's Edge",
            34 => "Shaft Bottom",
            35 => "Ice Passage",
            36 => "Ice Bear's Cave",
            37 => "Snowdrift",
            65 => "Ship's Deck",
            66 => "Cannon Deck",
            67 => "Open Water",
            69 => "Floating Plank",
            70 => "Island Shore (West)",
            71 => "Coconut Palm",
            72 => "Island Coast",
            73 => "Willy's Rest",
            74 => "The Bay",
            75 => "Blockhouse",
            76 => "Blockhouse Interior",
            77 => "Palisade Fence",
            78 => "Rope Ladder",
            79 => "Jungle Edge",
            80 => "Deep Jungle",
            81 => "Shelter on the Fence",
            83 => "Grassy Clearing",
            84 => "Jungle Path",
            85 => "Jungle Path",
            86 => "Jungle Path",
            87 => "The Chasm",
            88 => "Rock Wall",
            89 => "Rock Chimney",
            90 => "Chimney Ledge",
            91 => "The Summit",
            92 => "Broken Bridge",
            93 => "Rock Door",
            95 => "Pete's Body",
            96 => "The Catacomb",
            97 => "Rock Needle",
            119 => "Repair Depot Gate",
            120 => "Construction Site",
            121 => "Hovercraft Cockpit",
            122 => "Maintenance Unit",
            123 => "Lift",
            124 => "Boston Gate",
            125 => "Repair Hall",
            126 => "Driver's Stand",
            127 => "Platform",
            128 => "Baltimore Parking",
            129 => "District 6",
            130 => "Street",
            131 => "City Notice",
            132 => "East Street",
            133 => "Vehicle Yard",
            134 => "Subway Station",
            135 => "Inside the SRT",
            136 => "Info Service Lift",
            137 => "Terminal Hall",
            138 => "SRT Platform",
            139 => "City Centre",
            140 => "Vending Machine",
            141 => "Fountain",
            142 => "Subway",
            143 => "Museum, Scene I",
            144 => "Museum, Scene III",
            145 => "Museum Terminal",
            146 => "River Bank",
            147 => "Footprints",
            148 => "Fern Forest",
            149 => "River Mouth",
            150 => "Triceratops Meadow",
            151 => "Sabertooth Steppe",
            152 => "Nesting Ground",
            153 => "Baby Dinosaur's Rock",
            154 => "River",
            155 => "River Area",
            156 => "The Shallows",
            157 => "Sandbank",
            160 => "Mine",
            161 => "Rock Walls",
            162 => "Rail Tracks",
            163 => "Mine Cart",
            164 => "Dark Tunnel",
            165 => "Ore Station",
            166 => "Ore Heap",
            167 => "Coupling Works",
            168 => "Points Lever",
            169 => "The Stranger",
            170 => "Volcano Forge",
            171 => "Old Track",
            172 => "West/East Drift",
            173 => "Tunnel Entrance",
            174 => "The Smith",
            175 => "The Bridge",
            176 => "The Bridge",
            177 => "Beyond the Bridge",
            178 => "The Stairs",
            179 => "The Locked Door",
            180 => "Rat Hall",
            181 => "The Black Crypt",
            182 => "Labyrinth",
            183 => "Black Fog",
            184 => "Underground Lake",
            185 => "Underwater",
            188 => "Lake Bottom",
            190 => "North Shore",
            191 => "The Dead Bool",
            192 => "Inside the Bool",
            193 => "Metal Skin",
            194 => "Metal Skin",
            195 => "The Sluice",
            196 => "The Basin",
            198 => "Kathaura's Watch",
            199 => "The Black Way",
            200 => "Kathaura's Apartment",
            201 => "The Black City",
            202 => "Fortress Wall",
            203 => "Fortress Gate",
            204 => "North Street",
            205 => "The Pavilion",
            206 => "East Street",
            208 => "Market Street",
            210 => "Temple Street",
            212 => "Khalin's Table",
            214 => "Khalin's Table",
            233 => "The Pavilion Floor",
            234 => "Kal's Laboratory",
            235 => "Machine Hall",
            236 => "Kal's Chamber",
            238 => "The Spaceship",
            _ => "Somewhere",
        }
    }

    pub fn chapter_of(id: u16) -> u8 {
        match id {
            1..=26 => 1,
            27..=37 => 3,
            65..=97 => 4,
            119..=145 => 2,
            146..=157 => 5,
            160..=182 => 6,
            183..=198 => 7,
            199..=238 => 7,
            _ => 1,
        }
    }

    pub fn enter(&mut self, id: u16) {
        self.room = id;
        self.chapter = Self::chapter_of(id);
        let name = self.room_name(id);
        self.print("");
        self.print(&format!("== {name} =="));
        let desc = self.describe(id);
        self.print(&desc);
    }

    pub fn describe(&mut self, id: u16) -> String {
        match id {
            1 => {
                if self.flag("gong") {
                    "The entrance hall of the Hawthorne mansion. A broad staircase climbs to the gallery, a gilt-framed portrait watches you, and a chandelier hangs from a rope above the tiled floor. The front door stands to the south; the library is north, the drawing room west and the billiard room east.".into()
                } else {
                    "You stand in the entrance hall of the old Hawthorne mansion. A chandelier hangs above, a portrait dominates the wall, and doors lead north to the library, west to the drawing room and east to the billiard room. The front door is south.".into()
                }
            }
            2 => "The library smells of leather and dust. Shelves cover the walls, a couch stands near the fireplace, and a fine samurai sword rests on a stand. A door leads north into the billiard room and south back to the hall.".into(),
            3 => "The drawing room is dominated by a grand piano. A landscape painting hangs above it. The hall lies east.".into(),
            4 => "The billiard room. A green table fills the centre, cues stand in a rack, and trophies glint on a shelf. There is a secret feeling about the panelled walls. The library is south.".into(),
            5 => "You are inside the great fireplace. Soot coats the bricks, and one brick looks loose. It is cramped here.".into(),
            7 => "The bedroom. A huge four-poster bed with a plush canopy stands against the wall. The door opens south to the upstairs hall.".into(),
            8 => "The bathroom. A claw-footed bathtub, a washbasin and a toilet. A large spider sits on a sponge in the tub, and a web stretches across the toilet.".into(),
            9 => "The upstairs hall. A heavy table stands here, and above it a trapdoor leads to the attic. Bedroom, bathroom and kitchen open off this corridor; the stairs go down to the entrance hall.".into(),
            10 => "The kitchen. A scrubbed table, a cupboard and a flypaper strip. A door leads east to the hall and south to the nursery.".into(),
            11 => "The nursery. Shelves of toys and books line the walls, and a faded rug covers the floor. The kitchen is north.".into(),
            12 => "The attic. Crates, a wooden chest and a steamer trunk are stacked beneath the rafters. A door west leads to the storeroom; the stairs go down.".into(),
            13 => "The attic storeroom. Junk is piled everywhere, and an old kitchen buffet stands against the wall. The attic is east.".into(),
            14 => "The wine cellar. Racks of dusty bottles line the walls, each with a label. Stairs lead up to the house, and a low door opens west into a junk room.".into(),
            15 => "A cramped junk room under the house. Garden tools lean in a corner, and a heap of broken furniture and old crates fills the rest. The wine cellar is east.".into(),
            16 => "A stone cellar beneath the billiard room. A great stone block stands in the middle; a pickaxe leans against the wall. The floor sounds hollow in one place. Steps lead up.".into(),
            24 => "The black pit. The walls are carved with strange cryptograms, and a stone slot waits beside a broken lever. A monstrous face is carved above. A stair leads up; a passage descends to the stone circle.".into(),
            26 => "The secret cellar. Six stone blocks stand in a circle, each marked with a Roman numeral. Between them lie six coloured fields: white, yellow, green, red, blue and black. The air tastes of thunder.".into(),
            27 => "A rocky niche in the ice. A wooden crate, a barrel, a flag on a pole and a snow-covered boulder are here. The cold cuts through you. A path leads south.".into(),
            28 => "A slope of blue ice. You can climb north to the niche, or west toward a steeper slope.".into(),
            29 => "A steep slope. Handholds of frozen rock lead up, and the way east returns to the lower slope.".into(),
            32 => "The plateau. A huge monolith stands here, its surface carved with reliefs and a monstrous face with a single eye socket. The wind whips snow around it.".into(),
            33 => "The edge of a crater. Hot debris steams in the snow, and a deep shaft drops into darkness.".into(),
            34 => "The bottom of the shaft. A snow pile has collected here, and a passage leads east.".into(),
            35 => "An ice passage. A sign is frozen to the rock, and a barred gate blocks the way east.".into(),
            36 => "The ice bear's cave. A shrine stands at the far end, a shard glittering on it. The bear is gone.".into(),
            37 => "A snowdrift. Nothing here but snow, and the promise of Christmas.".into(),
            65 => "The deck of an English ship. An officer watches the horizon, a flag snaps overhead, and cannons line the rail. The sea heaves below.".into(),
            66 => "The cannon deck. Ochsen-Willy and Pocken-Pete work the gun. Pete limps badly. A cannonball rests in the rack.".into(),
            67 => "You are in the water. A wooden plank floats nearby. The ship is sinking.".into(),
            69 => "You cling to a floating plank. There is a knot hole in the middle, and the sea stretches away.".into(),
            70 => "The western shore of a tropical island. A rock cone and dense jungle block the interior; a coconut palm leans over the sand.".into(),
            71 => "You are up the coconut palm. A coconut hangs within reach, and the scratches on your arms sting.".into(),
            72 => "The island coast. The sea is empty.".into(),
            73 => "Behind a rock on the north shore lies Willy, the pickpocket. He is dead. His clothes are sodden.".into(),
            74 => "A sheltered bay. A sea chest floats out of reach in the water.".into(),
            75 => "A log blockhouse. The door is shut, and thick jungle presses close.".into(),
            76 => "Inside the blockhouse. A rough table, a bunk and a warning note. A bottle stands on the table.".into(),
            77 => "A palisade fence crowned with thorn bushes. A shelter is built against the fence, and a rope ladder hangs inside.".into(),
            78 => "You are on the rope ladder, between the thorns and the jungle floor.".into(),
            79 => "The jungle edge. A wall of blossom and fern. The air is heavy and sweet.".into(),
            80 => "The deep jungle. The perfumed blossoms make your head swim.".into(),
            81 => "A small shelter on top of the palisade. The wooden floor is loose.".into(),
            83 => "A grassy clearing beyond the fence. Mushrooms grow in the grass, and paths lead north and west.".into(),
            84 => "A jungle path. Ferns close in on both sides.".into(),
            85 => "A jungle path. Monkeys shriek overhead.".into(),
            86 => "A jungle path. The ground is soft and dark.".into(),
            87 => "A chasm splits the island. A rope winch and a raised wooden bridge stand on the far side. A crank with a wooden peg projects from the winch.".into(),
            88 => "A rock wall. A ledge is just above, and a plant grows from a crack.".into(),
            89 => "A narrow rock chimney. You can climb.".into(),
            90 => "A ledge high on the cliff. The view is dizzying.".into(),
            91 => "The summit. A chest stands open in the wind, and a shard blazes inside.".into(),
            92 => "The broken bridge hangs over the chasm.".into(),
            93 => "A rock door bars the way into the catacombs.".into(),
            95 => "Pete's body lies in the passage. His wounds are obvious, and he still wears his clothes and eyepatch.".into(),
            96 => "The catacomb. A shard glitters on a rock needle, guarded by a tentacled monster that demands Kal's sign.".into(),
            97 => "A rock needle. The shard rests on top.".into(),
            119 => "The gate of a repair depot. A speaker grille and a button are set into the wall.".into(),
            120 => "A construction site in the Catskill Mountains. Barriers, undergrowth and a hovercraft. A sign gives the site number.".into(),
            121 => "The hovercraft cockpit. The instrument panel, a diagram and the seat are here. The ignition waits for a key.".into(),
            122 => "The maintenance unit. A metal cover, a plastic box and a fuse.".into(),
            123 => "A lift platform. A sign lists the depot levels.".into(),
            124 => "The gate of the Boston depot. A speaker and a button.".into(),
            125 => "A vast repair hall. A giant repair vehicle looms over you, with a ladder, a platform and a driver's stand.".into(),
            126 => "The driver's stand of the repair vehicle. A tangle of controls, a lever, a button and a dead-man switch.".into(),
            127 => "A maintenance platform on the repair vehicle. Tongs lie just out of reach.".into(),
            128 => "A parking lot in Baltimore. A door and an intercom.".into(),
            129 => "District 6 of Baltimore. Concrete, signs and a stale wind.".into(),
            130 => "A street in Baltimore. The subway entrance is west, the city centre east.".into(),
            131 => "A public notice board. The city warns of oxygen rationing.".into(),
            132 => "An east street. Vehicles and machines stand idle.".into(),
            133 => "A vehicle yard. Rows of dead machines.".into(),
            134 => "A subway station. A cleaning robot waits by the turnstile, and an SRT car stands at the platform.".into(),
            135 => "Inside the SRT. A panel of numbered buttons waits for a destination.".into(),
            136 => "The info service lift. A card slot glows beside the door.".into(),
            137 => "A hall of computer terminals. A single terminal is active.".into(),
            138 => "The SRT platform.".into(),
            139 => "The city centre. A fountain plays in the square, robots bustle about, and a three-eyed Roboss guards a vending machine to the west.".into(),
            140 => "A vending machine. A button and a delivery slot. The Roboss blocks the way.".into(),
            141 => "The fountain. Water arcs from copper nozzles, and a control box is set into the base.".into(),
            142 => "A subway tunnel. The SRT waits.".into(),
            143 => "The first museum hall. Two great scenes are displayed.".into(),
            144 => "The second museum hall. Two more scenes stand in glass cases.".into(),
            145 => "The end of the museum. A terminal and an output slot face you.".into(),
            146 => "A river bank in prehistoric times. A tyrannosaurus blocks the way to a glittering shard. A steaming dung heap lies at your feet.".into(),
            147 => "Fossil footprints in the mud. Leaves are caught between them.".into(),
            148 => "A fern forest. A triceratops grazes among the fronds.".into(),
            149 => "The river mouth. The water is wide and brown.".into(),
            150 => "A meadow where a triceratops waits. The fern forest is south.".into(),
            151 => "The steppe. A sabertooth tiger gnaws a haunch of meat. A brontosaurus grazes to the west.".into(),
            152 => "A nesting ground. Eggs lie in the sand.".into(),
            153 => "A rock where a baby dinosaur sits clutching a shard.".into(),
            154 => "A bend in the river. Something moves under the water.".into(),
            155 => "A quiet stretch of river bank.".into(),
            156 => "The shallows. A findling rock and a sandbank lie ahead.".into(),
            157 => "A sandbank. A giant spider squats beside a banana palm.".into(),
            160 => "A mine worked by slaves. Rails run east into a tunnel, and an exhausted man lies at the south end.".into(),
            161 => "Rock walls glitter with ore.".into(),
            162 => "The rail tracks. A cart waits on the line.".into(),
            163 => "A mine cart. It can be filled and sent along the rails.".into(),
            164 => "A dark tunnel. You need light.".into(),
            165 => "The great ore station. Rails cross in a maze, and an opening leads down into the mountain.".into(),
            166 => "A heap of ore and stones.".into(),
            167 => "The coupling works. Heavy machinery waits.".into(),
            168 => "A points lever beside a dark shaft.".into(),
            169 => "A stranger stands by the rails. He watches you warily.".into(),
            170 => "The volcano forge. A great anvil, a casting mould and a furnace fill the cavern. The smith works here.".into(),
            171 => "An old stretch of track.".into(),
            172 => "A west/east drift in the mine.".into(),
            173 => "A tunnel entrance. The black crypt lies beyond.".into(),
            174 => "Tell, the smith, waits by the tunnel with an amulet in his hand.".into(),
            175 => "A stone bridge over a bottomless chasm. A beast crouches on the far side.".into(),
            176 => "The bridge. A steel ball rests in a hollow in the stone.".into(),
            177 => "Beyond the bridge. The beast whispers from the dark.".into(),
            178 => "Worn stairs climb toward a door. A cage hangs from the ceiling.".into(),
            179 => "A great door bars the way north. A keyhole waits in the lock.".into(),
            180 => "A hall of insects and rats. Exits lead north and south.".into(),
            181 => "The black crypt. A paper lies on a stone lectern. Six exits vanish into the dark.".into(),
            182 => "A labyrinth of black stone. Six passages gape in every wall.".into(),
            183 => "Black fog curls around your feet.".into(),
            184 => "An underground lake. The water is black and still. Something huge moves below.".into(),
            185 => "You are underwater. The lake bed slopes away into darkness.".into(),
            188 => "The lake bottom. Light flickers from a rock plate.".into(),
            190 => "The north shore of the lake. Sand, a rock wall and strange plants.".into(),
            191 => "The dead Bool lies on the lake bed, its glass-and-plastic body complete.".into(),
            192 => "Inside the Bool. A control panel with a button and a switch.".into(),
            193 => "You cling to the Bool's metal skin.".into(),
            194 => "High on the Bool's hull.".into(),
            195 => "The sluice. A steel gate, a grille and a control unit with a button, a key and a regulator.".into(),
            196 => "A flooded basin. Something moves in the water.".into(),
            198 => "Kathaura's Watch. A giant's chair, torches and a drawbar. The way north is barred by Kathaura.".into(),
            199 => "The black way. A great door without handle or keyhole blocks the passage north, and a seam invites a lever.".into(),
            200 => "Kathaura's apartment. A spell is cut into the wall beside a giant chair, and soot blackens the stone behind it.".into(),
            201 => "The black city. Towers of obsidian crowd overhead.".into(),
            202 => "The fortress wall. Through a window you see Khalin, enormous, peering down.".into(),
            203 => "The fortress gate. A giant door with a button.".into(),
            204 => "A north street of the black city.".into(),
            205 => "The pavilion on Khalin's table. A giant chair and a balustrade.".into(),
            206 => "An eastern street.".into(),
            208 => "A market street.".into(),
            210 => "A temple street.".into(),
            212 => "Khalin's table. The giantess's chair and glasses are here.".into(),
            214 => "You are still on Khalin's table.".into(),
            233 => "The floor of the pavilion. A stone slab looks loose.".into(),
            234 => "Kal's laboratory. Machines and apparatus crowd the chamber.".into(),
            235 => "The machine hall. A great cannon points at a platform.".into(),
            236 => "Kal's chamber. A glass wall separates you from a seated figure.".into(),
            238 => "A spaceship. Stars wheel beyond the viewport.".into(),
            _ => "You see nothing special.".into(),
        }
    }

    // ------------------------------------------------------------------
    // Command handling
    // ------------------------------------------------------------------

    pub fn command(&mut self, input: &str) {
        let raw = input.trim().to_lowercase();
        if raw.is_empty() {
            return;
        }
        self.moves += 1;
        let words: Vec<&str> = raw.split_whitespace().collect();
        let mut verb = words[0];
        let mut noun = words[1..].join(" ");

        // Multi-word verbs
        if words.len() >= 2 {
            match (words[0], words[1]) {
                ("pick", "up") => {
                    verb = "take";
                    noun = words[2..].join(" ");
                }
                ("put", "on") => {
                    verb = "wear";
                    noun = words[2..].join(" ");
                }
                ("take", "off") => {
                    verb = "remove";
                    noun = words[2..].join(" ");
                }
                ("look", "at") | ("look", "in") | ("look", "under") | ("look", "into") => {
                    verb = "look";
                    noun = words[2..].join(" ");
                }
                ("climb", "up") => {
                    verb = "climb";
                    noun = if words.len() > 2 { words[2..].join(" ") } else { "up".to_string() };
                }
                ("climb", "down") => {
                    verb = "down";
                    noun = String::new();
                }
                ("go", _) => {
                    if let Some(d) = normalize_dir(words[1]) {
                        self.go(d);
                        return;
                    }
                }
                ("walk", _) | ("move", _) => {
                    if let Some(d) = normalize_dir(words[1]) {
                        self.go(d);
                        return;
                    }
                }
                _ => {}
            }
        }

        // A bare direction is movement.
        if let Some(d) = normalize_dir(verb) {
            self.go(d);
            return;
        }

        match verb {
            "l" | "look" | "examine" | "x" | "inspect" | "search" => {
                if noun.is_empty() || noun == "around" || noun == "room" {
                    let d = self.describe(self.room);
                    self.print(&d);
                    self.room_hint_actions();
                } else {
                    self.interact("look", &noun, &raw);
                }
            }
            "take" | "get" | "grab" | "nimm" if !noun.is_empty() => self.interact("take", &noun, &raw),
            "drop" | "put" | "leave" if !noun.is_empty() => self.interact("drop", &noun, &raw),
            "open" if !noun.is_empty() => self.interact("open", &noun, &raw),
            "close" | "shut" if !noun.is_empty() => self.interact("close", &noun, &raw),
            "read" if !noun.is_empty() => self.interact("read", &noun, &raw),
            "push" | "press" if !noun.is_empty() => self.interact("press", &noun, &raw),
            "pull" if !noun.is_empty() => self.interact("pull", &noun, &raw),
            "turn" if !noun.is_empty() => self.interact("turn", &noun, &raw),
            "play" if !noun.is_empty() => self.interact("play", &noun, &raw),
            "use" if !noun.is_empty() => self.interact("use", &noun, &raw),
            "attach" | "tie" | "fix" | "fasten" | "bewestige" if !noun.is_empty() => {
                self.interact("attach", &noun, &raw)
            }
            "detach" | "dismantle" | "disassemble" | "zerlege" if !noun.is_empty() => {
                self.interact("detach", &noun, &raw)
            }
            "cut" | "slice" if !noun.is_empty() => self.interact("cut", &noun, &raw),
            "throw" | "toss" if !noun.is_empty() => self.interact("throw", &noun, &raw),
            "give" if !noun.is_empty() => self.interact("give", &noun, &raw),
            "ask" if !noun.is_empty() => self.interact("ask", &noun, &raw),
            "say" | "speak" if !noun.is_empty() => self.interact("say", &noun, &raw),
            "climb" if !noun.is_empty() => self.interact("climb", &noun, &raw),
            "jump" => self.interact("jump", &noun, &raw),
            "swim" => self.interact("swim", &noun, &raw),
            "dive" | "tauch" => self.interact("dive", &noun, &raw),
            "light" | "ignite" | "zünd" if !noun.is_empty() => self.interact("light", &noun, &raw),
            "extinguish" | "put out" if !noun.is_empty() => self.interact("extinguish", &noun, &raw),
            "eat" if !noun.is_empty() => self.interact("eat", &noun, &raw),
            "drink" if !noun.is_empty() => self.interact("drink", &noun, &raw),
            "dig" => self.interact("dig", &noun, &raw),
            "fill" if !noun.is_empty() => self.interact("fill", &noun, &raw),
            "pour" if !noun.is_empty() => self.interact("pour", &noun, &raw),
            "polish" | "rub" if !noun.is_empty() => self.interact("polish", &noun, &raw),
            "wear" if !noun.is_empty() => self.interact("wear", &noun, &raw),
            "remove" if !noun.is_empty() => self.interact("remove", &noun, &raw),
            "board" => self.interact("board", &noun, &raw),
            "drive" | "sail" | "steer" | "paddle" if !noun.is_empty() => self.interact("drive", &noun, &raw),
            "stand" if !noun.is_empty() => self.interact("stand", &noun, &raw),
            "limp" => self.interact("limp", &noun, &raw),
            "bury" | "begraben" => self.interact("bury", &noun, &raw),
            "insert" if !noun.is_empty() => self.interact("insert", &noun, &raw),
            "hold" if !noun.is_empty() => self.interact("hold", &noun, &raw),
            "knock" => self.interact("knock", &noun, &raw),
            "smell" => self.interact("smell", &noun, &raw),
            "destroy" | "break" | "smash" if !noun.is_empty() => self.interact("destroy", &noun, &raw),
            "wait" | "z" => self.interact("wait", &noun, &raw),
            "yes" | "y" | "ja" => self.answer_yes(),
            "no" | "n" | "nein" => self.answer_no(),
            "inventory" | "i" | "inv" => self.show_inventory(),
            "score" | "points" => self.print(&format!("Score: {} of {}", self.score, MAX_SCORE)),
            "help" | "?" => self.show_help(),
            "journal" | "diary" => self.show_journal(),
            "hint" => self.show_hint(),
            "quit" | "exit" => self.print("Close the window to leave the game."),
            _ => {
                // Unknown first word: try a noun-only look, else complain.
                self.interact("unknown", &raw, &raw);
            }
        }
    }

    fn show_inventory(&mut self) {
        if self.inv.is_empty() {
            self.print("You are carrying nothing.");
        } else {
            let mut names: Vec<&str> = self.inv.iter().map(|i| item_name(i)).collect();
            names.sort();
            names.dedup();
            self.print(&format!("You are carrying: {}.", names.join(", ")));
        }
        let shards: Vec<String> = (1..=6)
            .filter(|i| self.flag(&format!("shard{i}")))
            .map(|i| i.to_string())
            .collect();
        if shards.is_empty() {
            self.print("You carry no gem splinters.");
        } else {
            self.print(&format!("Gem splinters: {} of 6 ({}).", shards.len(), shards.join(", ")));
        }
    }

    fn show_help(&mut self) {
        self.print("Commands: LOOK, EXAMINE <thing>, TAKE <thing>, DROP <thing>, OPEN, CLOSE, READ,");
        self.print("PUSH, PULL, TURN, USE, ATTACH <a> TO <b>, DETACH <thing>, CUT, THROW, GIVE, ASK,");
        self.print("SAY <word>, CLIMB, JUMP, SWIM, DIVE, LIGHT, EAT, WEAR, WAIT, INVENTORY, SCORE.");
        self.print("Move with N, S, E, W, UP, DOWN. JOURNAL shows the diary notes; HINT gives a nudge.");
    }

    pub fn show_journal(&mut self) {
        self.print("--- From the diary of Harold Jugger ---");
        self.print("March 29, 1922 (page 16): 'I have hidden the lighter behind the piano in the drawing room. The wine of that year is the only petroleum that will burn cleanly.'");
        self.print("March 5, 1922 (page 5): 'Mother's name was MARTHA. The coin is sewn into the spine of the children's bible.'");
        self.print("July 5, 1927 (page 24): 'The priest of Kal was called AURELIUS. His code is three digits.'");
        self.print("July 15, 1927 (page 33):");
        self.print("  'When the eye is whole and the six stones stand,");
        self.print("   white for the oldest ice, red for the new world,");
        self.print("   green for the sea's long voyage, yellow for the dawn,");
        self.print("   blue for the ships that never came home,");
        self.print("   black for the mine where the fire is born.'");
        self.print("Stone circle combinations:");
        self.print("  ICE: white field, block I.   USA: red field, block III.");
        self.print("  SHIP: green field, block V.  PREHISTORY: yellow field, block II.");
        self.print("  MINE: black field, block VI (requires five shards).");
        self.print("(The original boxed game shipped a physical diary; these are its relevant entries.)");
    }

    fn show_hint(&mut self) {
        let h = self.hint_text();
        self.print(h);
    }

    fn room_hint_actions(&mut self) {
        // Not shown by default; the room description is enough.
    }

    // ------------------------------------------------------------------
    // Movement
    // ------------------------------------------------------------------

    pub fn go(&mut self, dir: &'static str) {
        if self.room == 182 {
            self.labyrinth_move(dir);
            return;
        }
        let (dest, msg): (Option<u16>, Option<String>) = match self.room {
            1 => match dir {
                "north" => (Some(2), None),
                "west" => (Some(3), None),
                "east" => (Some(4), None),
                "up" => (Some(9), None),
                "down" => (Some(5), Some("You climb into the great fireplace.".into())),
                _ => (None, None),
            },
            5 => match dir {
                "down" | "south" => {
                    if self.flag("brick_open") {
                        (Some(14), Some("You descend the hidden stair behind the fireplace into the wine cellar.".into()))
                    } else {
                        (Some(1), None)
                    }
                }
                "up" | "out" => (Some(1), None),
                _ => (None, None),
            },
            2 => match dir {
                "south" => (Some(1), None),
                "north" => (Some(4), None),
                _ => (None, None),
            },
            3 => match dir {
                "east" => (Some(1), None),
                _ => (None, None),
            },
            4 => match dir {
                "south" => (Some(2), None),
                "down" => {
                    if self.flag("billiard_passage") {
                        (Some(16), Some("You descend the hidden stair behind the panelling.".into()))
                    } else {
                        (None, Some("The panelling is solid. You would need to open a passage first.".into()))
                    }
                }
                _ => (None, None),
            },
            9 => match dir {
                "down" => (Some(1), None),
                "north" => (Some(7), None),
                "east" => (Some(8), None),
                "west" => (Some(10), None),
                "up" => {
                    if self.flag("trapdoor_open") {
                        (Some(12), Some("You climb through the trapdoor into the attic.".into()))
                    } else {
                        (None, Some("The trapdoor is closed and bolted.".into()))
                    }
                }
                _ => (None, None),
            },
            7 => match dir {
                "south" => (Some(9), None),
                _ => (None, None),
            },
            8 => match dir {
                "west" => (Some(9), None),
                _ => (None, None),
            },
            10 => match dir {
                "east" => (Some(9), None),
                "south" => (Some(11), None),
                _ => (None, None),
            },
            11 => match dir {
                "north" => (Some(10), None),
                _ => (None, None),
            },
            12 => match dir {
                "down" => (Some(9), None),
                "west" => (Some(13), None),
                _ => (None, None),
            },
            13 => match dir {
                "east" => (Some(12), None),
                _ => (None, None),
            },
            14 => match dir {
                "up" => (Some(1), None),
                "west" => (Some(15), None),
                _ => (None, None),
            },
            15 => match dir {
                "east" => (Some(14), None),
                _ => (None, None),
            },
            16 => match dir {
                "up" => (Some(4), None),
                "down" => {
                    if self.flag("cellar_plate_open") {
                        (Some(24), Some("You lower yourself through the hole into the black pit.".into()))
                    } else {
                        (None, Some("The floor is intact. There is no way down here yet.".into()))
                    }
                }
                _ => (None, None),
            },
            24 => match dir {
                "up" => (Some(16), None),
                "down" => {
                    if self.flag("pit_open") {
                        (Some(26), Some("The wall swings aside. You step into the stone circle.".into()))
                    } else {
                        (None, Some("The passage is still sealed.".into()))
                    }
                }
                _ => (None, None),
            },
            26 => match dir {
                "up" => (Some(24), None),
                _ => (None, None),
            },

            // ---- Ice world -------------------------------------------------
            27 => match dir {
                "south" => {
                    if self.flag("cold_protected") {
                        (Some(28), None)
                    } else {
                        (None, Some("The cold beyond the niche is deadly. You need boots, a fur coat with hood, gloves and goggles.".into()))
                    }
                }
                _ => (None, None),
            },
            28 => match dir {
                "north" => (Some(27), None),
                "west" => (Some(29), None),
                _ => (None, None),
            },
            29 => match dir {
                "east" => (Some(28), None),
                "up" | "west" => (Some(32), Some("You haul yourself up the last few metres to the plateau.".into())),
                _ => (None, None),
            },
            32 => match dir {
                "east" => (Some(29), None),
                "south" => {
                    if self.flag("monolith_gone") {
                        (Some(33), Some("You walk to the edge of the new crater.".into()))
                    } else {
                        (None, Some("The monolith blocks the way south.".into()))
                    }
                }
                _ => (None, None),
            },
            33 => match dir {
                "north" => (Some(32), None),
                "down" => {
                    if self.flag("rope_anchor") && self.flag("snow_soft") {
                        (Some(34), Some("You climb down the wrecking-ball rope until it runs out, then drop into the soft snow.".into()))
                    } else if self.flag("rope_anchor") {
                        (None, Some("The rope is anchored, but the bottom is hard rock. Throw snow down first.".into()))
                    } else {
                        (None, Some("The shaft is far too deep. You need a rope anchored to the debris.".into()))
                    }
                }
                _ => (None, None),
            },
            34 => match dir {
                "east" => (Some(35), None),
                "up" => (Some(33), None),
                _ => (None, None),
            },
            35 => match dir {
                "west" => (Some(34), None),
                "east" => {
                    if self.flag("icegate_open") {
                        (Some(36), Some("You push the gate open and enter the bear's cave.".into()))
                    } else {
                        (None, Some("A barred gate blocks the cave. It is locked.".into()))
                    }
                }
                _ => (None, None),
            },
            36 => match dir {
                "west" => (Some(35), None),
                _ => (None, None),
            },

            // ---- Ship and island ------------------------------------------
            65 => match dir {
                "east" => (Some(66), None),
                "west" | "north" | "south" => {
                    if self.flag("boatswain_orders") {
                        (Some(66), Some("You make your way to the cannon deck.".into()))
                    } else {
                        (Some(66), Some("The boatswain waves you toward the guns.".into()))
                    }
                }
                "down" => (Some(67), Some("You dive over the rail into the sea!".into())),
                _ => (None, None),
            },
            66 => match dir {
                "west" => (Some(65), None),
                "east" => (Some(65), Some("You return to the main deck.".into())),
                "down" => (Some(67), Some("You jump over the side!".into())),
                _ => (None, None),
            },
            67 => match dir {
                "up" | "north" | "south" => {
                    if self.flag("plank_reached") {
                        (Some(69), None)
                    } else {
                        (None, Some("You tread water. A plank floats nearby; try SWIM or CLIMB." .into()))
                    }
                }
                _ => (None, None),
            },
            69 => match dir {
                "east" | "west" | "north" | "south" => (None, None),
                _ => (None, None),
            },
            70 => match dir {
                "north" => (Some(73), None),
                "south" => (Some(74), None),
                "east" => {
                    if self.flag("thicket_cut") {
                        (Some(75), None)
                    } else {
                        (None, Some("A wall of thorny thicket blocks the way east. You could cut it with a sword.".into()))
                    }
                }
                "west" | "up" => {
                    if self.flag("palm_climbed") {
                        (Some(71), None)
                    } else {
                        (None, Some("The palm is too smooth to climb without a reason.".into()))
                    }
                }
                _ => (None, None),
            },
            71 => match dir {
                "down" => (Some(70), None),
                _ => (None, None),
            },
            73 => match dir {
                "south" => (Some(70), None),
                _ => (None, None),
            },
            74 => match dir {
                "north" => (Some(70), None),
                "east" => (Some(75), None),
                _ => (None, None),
            },
            75 => match dir {
                "north" => {
                    if self.flag("thicket_cut") {
                        (Some(77), None)
                    } else {
                        (None, Some("Thick jungle blocks the way north. Cut a path with a sword.".into()))
                    }
                }
                "south" | "west" => (Some(70), None),
                "in" | "enter" => (Some(76), None),
                _ => (None, None),
            },
            76 => match dir {
                "out" => (Some(75), None),
                "south" => (Some(75), None),
                _ => (None, None),
            },
            77 => match dir {
                "east" => {
                    if self.flag("fence_crossed") {
                        (Some(83), None)
                    } else {
                        (None, Some("The fence is too high. You would have to climb.".into()))
                    }
                }
                "up" => {
                    if self.flag("fence_ladder") {
                        (Some(81), None)
                    } else {
                        (None, Some("The thorns cut your hands. You need a way over.".into()))
                    }
                }
                "west" | "south" => (Some(75), None),
                _ => (None, None),
            },
            81 => match dir {
                "down" => (Some(83), Some("You climb down inside the fence.".into())),
                "up" | "out" => (Some(77), None),
                _ => (None, None),
            },
            83 => match dir {
                "north" => {
                    if self.flag("admirals_blessing") || self.flag("antidote_eaten") {
                        (Some(84), None)
                    } else if self.flag("admiral_met") {
                        (Some(80), Some("You step into the jungle. The blossoms overwhelm you...".into()))
                    } else {
                        (Some(80), None)
                    }
                }
                "west" | "south" => (Some(77), None),
                "east" => (None, None),
                _ => (None, None),
            },
            80 => match dir {
                "south" => (Some(83), None),
                "north" => {
                    if self.flag("admirals_blessing") {
                        (Some(84), None)
                    } else {
                        (Some(26), Some("The perfume of the blossoms overwhelms you. You wake in the secret cellar!".into()))
                    }
                }
                _ => (None, None),
            },
            84 => match dir {
                "north" => (Some(85), None),
                "south" => (Some(80), None),
                _ => (None, None),
            },
            85 => match dir {
                "north" => (Some(86), None),
                "south" => (Some(84), None),
                _ => (None, None),
            },
            86 => match dir {
                "north" => (Some(87), None),
                "south" => (Some(85), None),
                _ => (None, None),
            },
            87 => match dir {
                "west" => (Some(86), None),
                "north" | "east" => {
                    if self.flag("bridge_lowered") {
                        (Some(88), Some("You cross the bridge.".into()))
                    } else {
                        (None, Some("The bridge is raised and the chasm is uncrossable.".into()))
                    }
                }
                _ => (None, None),
            },
            88 => match dir {
                "east" => (Some(87), None),
                "north" => {
                    if self.flag("rockdoor_open") {
                        (Some(93), None)
                    } else {
                        (None, Some("The rock door beyond the wall is shut fast.".into()))
                    }
                }
                "up" | "west" => (Some(89), None),
                _ => (None, None),
            },
            89 => match dir {
                "up" => (Some(90), None),
                "down" | "east" => (Some(88), None),
                _ => (None, None),
            },
            90 => match dir {
                "up" => (Some(91), None),
                "down" => (Some(89), None),
                _ => (None, None),
            },
            93 => match dir {
                "north" | "in" => {
                    if self.flag("rockdoor_open") {
                        (Some(95), None)
                    } else {
                        (None, Some("The rock door is shut fast.".into()))
                    }
                }
                "east" => (Some(88), None),
                _ => (None, None),
            },
            95 => match dir {
                "north" => {
                    if self.flag("pete_disguise") {
                        (Some(96), None)
                    } else {
                        (None, Some("A tentacle lashes out of the dark. You need Kal's sign - or a convincing disguise.".into()))
                    }
                }
                "south" => (Some(93), None),
                _ => (None, None),
            },
            96 => match dir {
                "south" => (Some(95), None),
                _ => (None, None),
            },

            // ---- USA ------------------------------------------------------
            120 => match dir {
                "west" => (Some(121), None),
                "east" => (Some(122), None),
                _ => (None, None),
            },
            121 => match dir {
                "west" => (Some(120), None),
                "east" => (Some(122), None),
                _ => (None, None),
            },
            122 => match dir {
                "west" => (Some(121), None),
                _ => (None, None),
            },
            124 => match dir {
                "north" | "in" => {
                    if self.flag("boston_entered") {
                        (Some(125), None)
                    } else {
                        (None, Some("The gate is shut. Use the intercom.".into()))
                    }
                }
                "south" | "out" => (Some(121), Some("You climb back into the hovercraft.".into())),
                _ => (None, None),
            },
            125 => match dir {
                "north" => (Some(126), None),
                "east" => (Some(127), None),
                "south" => (Some(124), None),
                _ => (None, None),
            },
            126 => match dir {
                "south" => (Some(125), None),
                _ => (None, None),
            },
            127 => match dir {
                "west" => (Some(125), None),
                _ => (None, None),
            },
            128 => match dir {
                "west" => (Some(129), None),
                "east" => (Some(130), None),
                _ => (None, None),
            },
            129 => match dir {
                "east" => (Some(130), None),
                _ => (None, None),
            },
            130 => match dir {
                "west" => (Some(139), None),
                "east" => (Some(132), None),
                "down" => (Some(134), None),
                _ => (None, None),
            },
            132 => match dir {
                "west" => (Some(130), None),
                _ => (None, None),
            },
            134 => match dir {
                "up" => (Some(130), None),
                "east" => {
                    if self.flag("srt_door_open") {
                        (Some(135), None)
                    } else {
                        (None, Some("The SRT door is closed. Press the button.".into()))
                    }
                }
                _ => (None, None),
            },
            135 => match dir {
                "out" => (Some(134), None),
                _ => (None, None),
            },
            136 => match dir {
                "north" => {
                    if self.flag("card_inserted") {
                        (Some(137), None)
                    } else {
                        (None, Some("The lift door is locked. A card slot glows beside it.".into()))
                    }
                }
                _ => (None, None),
            },
            137 => match dir {
                "south" => (Some(136), None),
                _ => (None, None),
            },
            139 => match dir {
                "east" => (Some(130), None),
                "west" => (Some(140), None),
                _ => (None, None),
            },
            140 => match dir {
                "east" => (Some(139), None),
                _ => (None, None),
            },
            143 => match dir {
                "north" => (Some(144), None),
                "south" => (Some(139), None),
                _ => (None, None),
            },
            144 => match dir {
                "north" => (Some(145), None),
                "south" => (Some(143), None),
                _ => (None, None),
            },
            145 => match dir {
                "south" => (Some(144), None),
                _ => (None, None),
            },

            // ---- Prehistoric ---------------------------------------------
            146 => match dir {
                "south" => (Some(147), None),
                "east" => (Some(154), None),
                "west" => (Some(155), None),
                "north" => {
                    if self.flag("trex_lured") {
                        (Some(153), Some("The tyrannosaurus is busy with the meat. You slip past.".into()))
                    } else {
                        (None, Some("The tyrannosaurus roars and blocks the way.".into()))
                    }
                }
                _ => (None, None),
            },
            147 => match dir {
                "north" => (Some(146), None),
                "south" => (Some(150), None),
                _ => (None, None),
            },
            150 => match dir {
                "north" => (Some(147), None),
                "south" => (Some(151), None),
                "west" => (Some(153), None),
                _ => (None, None),
            },
            151 => match dir {
                "north" => (Some(150), None),
                "west" => (Some(155), None),
                _ => (None, None),
            },
            153 => match dir {
                "south" => (Some(150), None),
                "east" => (Some(146), None),
                _ => (None, None),
            },
            154 => match dir {
                "west" => (Some(146), None),
                _ => (None, None),
            },
            155 => match dir {
                "east" => (Some(146), None),
                "west" | "south" => (Some(156), None),
                "north" => (Some(151), None),
                _ => (None, None),
            },
            156 => match dir {
                "east" => (Some(155), None),
                "west" => {
                    if self.flag("sandbank_reached") {
                        (Some(157), None)
                    } else {
                        (None, Some("The sandbank is across the water. You need a line.".into()))
                    }
                }
                _ => (None, None),
            },
            157 => match dir {
                "east" => (Some(156), None),
                _ => (None, None),
            },

            // ---- Mine -----------------------------------------------------
            160 => match dir {
                "north" => (Some(162), None),
                "east" => (Some(165), None),
                "south" => (Some(162), Some("You walk to the exhausted man at the south end.".into())),
                _ => (None, None),
            },
            162 => match dir {
                "north" => (Some(160), None),
                "south" => (Some(160), Some("The exhausted man lies here.".into())),
                "west" => (Some(161), None),
                "east" => (Some(165), None),
                "up" => (Some(163), Some("You climb into the mine cart.".into())),
                _ => (None, None),
            },
            165 => match dir {
                "west" => (Some(160), None),
                "north" => (Some(171), None),
                "east" => (Some(169), None),
                "south" => (Some(166), None),
                "up" => (Some(163), Some("You climb into a cart.".into())),
                _ => (None, None),
            },
            171 => match dir {
                "north" => (Some(172), None),
                "south" => (Some(165), None),
                _ => (None, None),
            },
            172 => match dir {
                "north" => (Some(173), None),
                "south" => (Some(171), None),
                _ => (None, None),
            },
            173 => match dir {
                "north" => (Some(174), None),
                "south" => (Some(172), None),
                _ => (None, None),
            },
            169 => match dir {
                "west" => (Some(165), None),
                "east" => {
                    if self.flag("tell_met") {
                        (Some(170), None)
                    } else {
                        (None, Some("The stranger blocks the way east. Perhaps answer him first.".into()))
                    }
                }
                _ => (None, None),
            },
            170 => match dir {
                "west" => (Some(169), None),
                _ => (None, None),
            },
            174 => match dir {
                "north" => (Some(175), None),
                "south" => (Some(173), None),
                _ => (None, None),
            },
            175 => match dir {
                "north" | "east" => (Some(176), None),
                "south" => (Some(174), None),
                _ => (None, None),
            },
            176 => match dir {
                "north" | "east" => (Some(177), None),
                "south" | "west" => (Some(175), None),
                _ => (None, None),
            },
            177 => match dir {
                "east" | "north" => (Some(178), None),
                "west" | "south" => (Some(176), None),
                _ => (None, None),
            },
            178 => match dir {
                "east" | "up" => (Some(179), None),
                "west" | "down" => (Some(177), None),
                _ => (None, None),
            },
            179 => match dir {
                "north" => {
                    if self.flag("door_unlocked") {
                        (Some(181), None)
                    } else {
                        (None, Some("The door is locked. The keyhole is round and very large.".into()))
                    }
                }
                "south" => (Some(178), None),
                _ => (None, None),
            },
            181 => match dir {
                "east" | "north" => (Some(182), None),
                "south" => (Some(179), None),
                _ => (None, None),
            },
            182 => (None, None),
            184 => match dir {
                "dive" | "down" | "south" => (Some(185), None),
                "north" => (Some(190), None),
                _ => (None, None),
            },
            185 => match dir {
                "north" => (Some(190), None),
                "south" | "east" => (Some(191), None),
                "up" | "west" => (Some(184), None),
                _ => (None, None),
            },
            190 => match dir {
                "dive" | "down" | "south" => (Some(191), None),
                "north" => (Some(184), None),
                _ => (None, None),
            },
            191 => match dir {
                "in" | "enter" | "north" => {
                    if self.flag("bool_open") || self.flag("cube_charged") {
                        (Some(192), None)
                    } else {
                        (None, Some("The Bool's mouth is full of sharp teeth. You need light strong enough to see by.".into()))
                    }
                }
                "up" | "south" => (Some(185), None),
                _ => (None, None),
            },
            192 => match dir {
                "out" => (Some(190), None),
                "up" => (Some(193), None),
                _ => (None, None),
            },
            193 => match dir {
                "up" => (Some(194), None),
                "down" | "in" => (Some(192), None),
                _ => (None, None),
            },
            194 => match dir {
                "up" | "north" => (Some(190), None),
                "down" => (Some(193), None),
                _ => (None, None),
            },
            195 => match dir {
                "north" => (Some(198), None),
                "east" => (Some(196), None),
                _ => (None, None),
            },
            196 => match dir {
                "west" => (Some(195), None),
                _ => (None, None),
            },
            198 => match dir {
                "north" => {
                    if self.flag("kathaura_trapped") {
                        (Some(199), None)
                    } else {
                        (None, Some("Kathaura blocks the way.".into()))
                    }
                }
                "south" => (Some(195), None),
                _ => (None, None),
            },
            188 => match dir {
                "down" => {
                    if self.flag("plate_open") {
                        (Some(195), None)
                    } else {
                        (None, Some("The rock plate is closed fast.".into()))
                    }
                }
                "south" | "dive" => (Some(191), None),
                "up" | "north" => (Some(190), None),
                _ => (None, None),
            },
            199 => match dir {
                "north" => {
                    if self.flag("blackway_door") {
                        (Some(200), None)
                    } else {
                        (None, Some("A great door blocks the black way. It has no handle, only a seam.".into()))
                    }
                }
                "south" => (Some(198), None),
                _ => (None, None),
            },
            200 => match dir {
                "north" | "east" => (Some(202), None),
                "south" => (Some(199), None),
                _ => (None, None),
            },
            202 => match dir {
                "north" => (Some(203), None),
                "south" => (Some(200), None),
                _ => (None, None),
            },
            203 => match dir {
                "north" => {
                    if self.flag("blackgate_open") {
                        (Some(201), Some("You pass through the gate into the black city.".into()))
                    } else {
                        (None, Some("The gate is shut. Khalin must open it.".into()))
                    }
                }
                "south" => (Some(202), None),
                _ => (None, None),
            },
            204 => match dir {
                "east" => (Some(206), None),
                _ => (None, None),
            },
            206 => match dir {
                "west" => (Some(204), None),
                "south" => (Some(208), None),
                _ => (None, None),
            },
            208 => match dir {
                "north" => (Some(206), None),
                "west" => (Some(201), None),
                "east" => (Some(210), None),
                _ => (None, None),
            },
            210 => match dir {
                "west" => (Some(208), None),
                _ => (None, None),
            },
            201 => match dir {
                "north" => (Some(205), None),
                "south" => (Some(203), None),
                "east" => (Some(208), None),
                _ => (None, None),
            },
            205 => match dir {
                "south" => (Some(201), None),
                "down" => (Some(233), None),
                _ => (None, None),
            },
            233 => match dir {
                "down" => {
                    if self.flag("pavilion_plate_open") {
                        (Some(234), None)
                    } else {
                        (None, Some("The stone floor is solid.".into()))
                    }
                }
                "up" => (Some(205), None),
                _ => (None, None),
            },
            234 => match dir {
                "north" => (Some(235), None),
                "up" => (Some(233), None),
                _ => (None, None),
            },
            235 => match dir {
                "north" => {
                    if self.flag("kal_switch_ready") {
                        (Some(236), None)
                    } else {
                        (None, Some("The glass wall has no power. A key switch waits in the machine hall.".into()))
                    }
                }
                "south" => (Some(234), None),
                _ => (None, None),
            },
            236 => match dir {
                "south" => (Some(235), None),
                "north" => {
                    if self.flag("kal_done") {
                        (Some(238), None)
                    } else {
                        (None, None)
                    }
                }
                _ => (None, None),
            },
            238 => (None, None),
            _ => (None, None),
        };

        match dest {
            Some(d) => {
                if let Some(m) = msg {
                    self.print(&m);
                }
                self.enter(d);
                self.arrive(d);
            }
            None => {
                if let Some(m) = msg {
                    self.print(&m);
                } else {
                    self.print("You can't go that way.");
                }
            }
        }
    }

    fn arrive(&mut self, id: u16) {
        match id {
            1 => {
                if !self.flag("gong") && self.moves >= 4 {
                    self.set("gong");
                    self.print("A deep gong sounds. Someone knocks at the front door.");
                }
            }
            9 => {
                if !self.flag("upstairs_seen") {
                    self.set("upstairs_seen");
                }
            }
            120 => {
                if !self.flag("usa_arrived") {
                    self.set("usa_arrived");
                    self.award(50);
                    self.print("The world dissolves in a cascade of colours. You materialise on a construction site in the Catskill Mountains.");
                }
            }
            65 => {
                if !self.flag("ship_arrived") {
                    self.set("ship_arrived");
                    self.print("An English man-of-war. The boatswain shouts that the French are coming and waves you toward the guns.");
                }
            }
            66 => {
                if !self.flag("willy_steal") {
                    self.set("willy_steal");
                    if self.has_shard(1) {
                        self.lose_shard(1);
                    }
                    self.print("Ochsen-Willy bumps into you and is suddenly very apologetic. Your pocket feels lighter - one gem splinter is gone!");
                }
            }
            70 => {
                if !self.flag("island_landed") {
                    self.set("island_landed");
                    self.print("Your plank grinds onto a tropical shore. The sea behind you is empty where the ship was.");
                }
            }
            150 => {
                if self.flag("bananas_dropped") && !self.flag("tiger_distracted") {
                    self.set("tiger_distracted");
                    self.print("A brontosaurus crashes out of the ferns, strips the bananas from the steppe and drives the sabertooth tiger away. The meat lies abandoned.");
                }
            }
            151 => {
                if self.flag("meat_dropped") && !self.flag("trex_lured") {
                    self.set("trex_lured");
                    self.print("The tyrannosaurus stamps out of the ferns, sniffs the meat - and the triceratops charges. Wounded, the tyrant limps away. The way north is open.");
                }
            }
            171 => {
                if !self.flag("slave_quarters") {
                    self.set("slave_quarters");
                    self.award(25);
                    self.print("You pass through the slaves' quarters. No one challenges a blackened, weary worker.");
                }
            }
            174 => {
                if !self.flag("amulet_offered") {
                    self.set("amulet_offered");
                    self.print("Tell the smith waits by the tunnel. He holds out a bronze amulet: 'Take it. Pit will guide you where I cannot.'");
                }
            }
            190 => {
                if !self.flag("north_shore_seen") {
                    self.set("north_shore_seen");
                    self.award(10);
                    self.print("You surface at the north shore, gasping. The lake lies black and still behind you.");
                }
            }
            201 => {
                if !self.flag("city_entered") {
                    self.set("city_entered");
                    self.award(10);
                    self.print("You step through the gate into the black city. Obsidian towers crowd overhead, and somewhere a giant bell tolls.");
                }
            }
            80 => {
                if !self.flag("admiral_met") {
                    self.set("admiral_met");
                    self.print("Admiral Atterson blocks the jungle path, clutching his stomach. He eyes you warily.");
                }
            }
            91 => {
                if !self.flag("summit_seen") {
                    self.set("summit_seen");
                    self.award(10);
                    self.print("You pull yourself onto the summit, winded and scratched.");
                }
            }
            27 => {
                if !self.flag("ice_arrived") {
                    self.set("ice_arrived");
                    self.print("Cold bites at you. You are in a rock niche at the edge of an ice world.");
                }
            }
            29 => {
                if !self.flag("slope_seen") {
                    self.set("slope_seen");
                    self.award(20);
                    self.print("You reach the steep slope, panting in the thin cold air.");
                }
            }
            36 => {
                if self.flag("torch_lit") && !self.flag("bear_gone") {
                    self.set("bear_gone");
                    self.print("The ice bear sniffs the burning torch, growls, and lumbers away into the dark.");
                }
            }
            125 => {
                if !self.flag("repair_hall_seen") {
                    self.set("repair_hall_seen");
                    self.award(10);
                    self.print("You climb down into the repair hall.");
                }
            }
            134 => {
                if !self.flag("subway_seen") {
                    self.set("subway_seen");
                    self.award(10);
                    self.print("You descend into the subway station.");
                }
            }
            143 => {
                if !self.flag("museum_seen") {
                    self.set("museum_seen");
                    self.award(25);
                    self.print("You walk the length of the Historical Museum, studying the scenes of the six worlds.");
                }
            }
            139 => {
                if !self.flag("city_centre_seen") {
                    self.set("city_centre_seen");
                }
            }
            26 => {
                self.print("The six coloured fields glow faintly. Use STAND ON <colour> and PRESS BLOCK <numeral> to travel.");
            }
            _ => {}
        }
    }

    pub fn hint_text(&self) -> &'static str {
        if self.room <= 26 {
            if !self.has("shard") && !self.flag("shard1") {
                if !self.flag("envelope_open") {
                    return "HINT: Take the samurai sword in the library and cut the cord of the envelope; the envelope contains your first shard.";
                }
                if !self.has("shard") {
                    return "HINT: Open the cut envelope and take the first shard, then explore the house for the way down.";
                }
                if !self.flag("billiard_passage") {
                    return "HINT: In the billiard room, knock the marble into a pocket to open the secret passage. Before that, solve the piano and the plaque in the hall.";
                }
                if !self.flag("stone_circle") {
                    return "HINT: In the cellar, dig with the pickaxe, pull the floor plate, then use the broken lever's handle in the black pit.";
                }
                return "HINT: Use the stone circle: STAND ON <colour>, then PRESS BLOCK <numeral>. Read JOURNAL for the combinations.";
            }
            return "HINT: You are in the mansion. Explore the library, drawing room, billiard room, kitchen, bedroom, bathroom and cellar.";
        }
        match self.chapter {
            2 => "HINT: At the construction site, search the bushes and the cockpit, damage the maintenance unit, then drive to Boston. In Baltimore, use the intercom and the fountain puzzle to get a code card.",
            3 => "HINT: Wear all five pieces of winter clothing, make a torch, climb to the monolith, crack it with snow and the shard, then descend, light the torch and open the gate for the ice bear.",
            4 => "HINT: Build a sail and later a billiard-cue mast, reach the island, befriend the admiral with spices, cross the fence, and disguise yourself as Pete in the catacomb.",
            5 => "HINT: Use leaves to carry the dung, lure the tyrannosaurus with meat, distract the sabertooth with bananas, and scare the baby dinosaur with the spider.",
            6 => "HINT: Use the cart and rope, revive the slave with the elixir, recover the sixth shard, have the Eye forged, then solve the bridge and labyrinth with Pit's riddles.",
            7 => "HINT: Trap Kathaura in the sluice, blacken yourself with soot, break Khalin's glasses, descend into Kal's temple, and answer Kal's questions.",
            _ => "HINT: Explore and examine everything.",
        }
    }
}

fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut out = Vec::new();
    if text.is_empty() {
        out.push(String::new());
        return out;
    }
    for paragraph in text.split('\n') {
        let mut line = String::new();
        for word in paragraph.split_whitespace() {
            if line.is_empty() {
                line.push_str(word);
            } else if line.len() + 1 + word.len() <= width {
                line.push(' ');
                line.push_str(word);
            } else {
                out.push(line);
                line = word.to_string();
            }
        }
        if !line.is_empty() {
            out.push(line);
        }
    }
    out
}
