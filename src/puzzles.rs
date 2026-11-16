//! All puzzle logic and object interactions.
//!
//! The room graph lives in `game.rs`; this module supplies the verbs.  It
//! follows Dominik's 1992 walkthrough (IF Archive) step for step, but with
//! English prose.  Score awards match the original walkthrough values.

use crate::game::{item_name, Game};

fn m(s: &str, words: &[&str]) -> bool {
    words.iter().any(|w| s.contains(w))
}

fn strip(s: &str) -> String {
    let mut t = s.trim().to_lowercase();
    loop {
        let before = t.clone();
        for p in [
            "at ", "the ", "a ", "an ", "on ", "in ", "into ", "to ", "with ", "under ",
            "behind ", "down ", "up ", "some ", "my ",
        ] {
            if let Some(rest) = t.strip_prefix(p) {
                t = rest.trim().to_string();
            }
        }
        if t == before {
            break;
        }
    }
    t
}

impl Game {
    // ------------------------------------------------------------------
    // Dispatch
    // ------------------------------------------------------------------

    pub fn interact(&mut self, verb: &str, noun: &str, _raw: &str) {
        let n = strip(noun);
        if verb == "unknown" {
            self.examine(&n);
            return;
        }
        match verb {
            "look" => self.examine(&n),
            "take" => self.pick_up(&n),
            "drop" | "put" => self.put_down(&n),
            "open" => self.open(&n),
            "close" => self.close(&n),
            "read" => self.read(&n),
            "press" => self.press(&n),
            "pull" => self.pull(&n),
            "turn" => self.turn(&n),
            "play" => self.play(&n),
            "use" => self.use_on(&n),
            "attach" => self.attach_cmd(&n),
            "detach" => self.detach(&n),
            "cut" => self.cut(&n),
            "throw" => self.throw(&n),
            "give" => self.give(&n),
            "ask" => self.ask(&n),
            "say" => self.say(&n),
            "climb" => self.climb(&n),
            "jump" => self.jump(&n),
            "swim" => self.swim(&n),
            "dive" => self.dive(&n),
            "light" => self.light(&n),
            "eat" => self.eat(&n),
            "dig" => self.dig(&n),
            "fill" => self.fill(&n),
            "pour" => self.pour(&n),
            "wear" => self.wear(&n),
            "remove" => self.remove_item(&n),
            "destroy" => self.destroy(&n),
            "wait" => self.wait(),
            "stand" => self.stand(&n),
            "limp" => self.limp(&n),
            "bury" => self.bury(&n),
            "drive" | "sail" | "steer" => self.drive(&n),
            "insert" => self.insert(&n),
            "hold" => self.hold(&n),
            "show" => self.show(&n),
            "listen" => self.ask("pit"),
            "smell" => self.print("The air here has its own smell - dust, salt, ice or oil, depending on the world."),
            "knock" => self.knock(),
            "extinguish" => self.extinguish(&n),
            "polish" | "rub" => self.polish(&n),
            "board" => self.board(&n),
            "drink" => self.drink(&n),
            "answer" => self.say(&n),
            a if a == "yes" || a == "y" || a == "ja" => self.answer_yes(),
            a if a == "no" || a == "nein" => self.answer_no(),
            _ => self.print("I don't know how to do that."),
        }
    }

    // ------------------------------------------------------------------
    // Looking at things
    // ------------------------------------------------------------------

    fn examine(&mut self, n: &str) {
        if n.is_empty() {
            let d = self.describe(self.room);
            self.print(&d);
            return;
        }
        if self.examine_room(n) {
            return;
        }
        if self.examine_later(n) {
            return;
        }

        // Things that may be in the inventory.
        let checks: &[(&str, &[&str], &str)] = &[
            ("diary", &["diary"], "The diary is bound in black leather. Its pages hold the dates 29 March 1922, 5 March 1922, 5 July 1927 and 15 July 1927. Use JOURNAL to read the entries that matter."),
            ("letter", &["letter"], "A letter in a shaky hand: 'Harold - the Eye of Kal was never a myth. Six splinters, six worlds. The stone circle in the cellar is the door. Trust the diary. - A.'"),
            ("envelope", &["envelope"], "A heavy envelope addressed to you in a dead man's handwriting. The cord has been cut."),
            ("blade", &["blade"], "The blade of a samurai sword. It is keen, and its tang will fit into a wooden handle."),
            ("sword", &["sword"], "A samurai sword, assembled from blade, handle and a little patience. It cuts rope, thorns and coconuts."),
            ("lighter", &["lighter"], "A brass petrol lighter. It is filled with 1922 petroleum and strikes a clean flame."),
            ("candle", &["candle"], "A tall candle from the chandelier. It gives a steady light when lit."),
            ("torch", &["torch"], "A club wrapped in a flag and soaked in whale oil. It will burn for a long time."),
            ("bottle", &["bottle of 1922", "petroleum", "1922"], "A dusty bottle of 1922 petroleum. The diary calls this vintage the only petroleum that burns cleanly."),
            ("jar", &["jar"], "A jam jar. It has a lid and is currently empty."),
            ("wax", &["wax"], "A tube of floor wax. It makes floors slippery and spiders unhappy."),
            ("sponge", &["sponge", "washcloth"], "A soft sponge. It smells faintly of wax."),
            ("toothbrush", &["toothbrush"], "A stiff toothbrush. Just the thing for polishing a waxed floor."),
            ("cloth", &["plush", "cloth", "towel"], "A plush cloth. A stitched label reads CES."),
            ("coin", &["coin"], "An old silver coin. One side bears a portrait, the other a crown."),
            ("key", &["small key", "little key"], "A small brass key, too small for a door and just right for a piano lock."),
            ("bible", &["bible"], "A children's bible. Something rattles in the spine."),
            ("marble", &["marble", "ball"], "A marble ball, heavy and smooth. It fits a billiard pocket exactly."),
            ("pickaxe", &["pickaxe"], "A miner's pickaxe, the handle dark with use."),
            ("hook", &["hook"], "A strong iron hook from the garden tools."),
            ("cue", &["cue", "billiard"], "A billiard cue. It can be unscrewed into tip, shaft and handle."),
            ("grapple", &["grapple"], "An iron hook lashed to a wooden shaft - a grappling hook."),
            ("boathook", &["boathook", "boat hook"], "A long boat hook: grappling hook, shaft and handle."),
            ("wrecking", &["wrecking"], "A heavy grappling hook on a long rope - a wrecking ball."),
            ("rammer", &["rammer"], "A powder rammer for a cannon."),
            ("powder", &["powder"], "Coarse black gunpowder. It smells of sulphur and the sea."),
            ("tin", &["tin"], "A dented breakfast tin. Something metal shifts inside."),
            ("hammer", &["hammer"], "A heavy hammer."),
            ("crowbar", &["crowbar"], "A bent steel crowbar."),
            ("passkey", &["passkey", "pass key"], "A universal passkey for service panels."),
            ("fuse", &["fuse"], "A ceramic fuse."),
            ("plug", &["plug"], "A heavy electric plug with a threaded collar."),
            ("tongs", &["tongs"], "A pair of long service tongs."),
            ("card", &["code card", "card"], "A blank code card from the vending machine."),
            ("card_coded", &["coded", "punched"], "A code card with a punched code."),
            ("quicksand", &["quicksand"], "A handful of dry quicksand. Cleaning robots find it irresistible."),
            ("bucket", &["bucket"], "A cleaner's bucket. It can be filled with water."),
            ("club", &["club"], "A heavy wooden club. With a flag and oil it becomes a torch."),
            ("flag", &["flag"], "A strip of cloth, perhaps a flag. It would burn well."),
            ("boots", &["boots"], "Thick fur-lined boots."),
            ("coat", &["coat", "mantle"], "A heavy fur coat with a deep hood and lined gloves in its pockets."),
            ("goggles", &["goggles"], "Snow goggles with darkened glass."),
            ("key", &["iron key", "icekey"], "A big iron key, cold to the touch."),
            ("coconut", &["coconut"], "A large coconut. Its shell is hard enough to break thorns."),
            ("stone", &["stone"], "A smooth heavy stone."),
            ("bottle_maggi", &["maggi"], "A bottle of Maggi seasoning, nearly empty. Lovage in a bottle, the admiral would say."),
            ("cork", &["cork", "crown cork"], "A crown cork from a bottle. It makes a passable cup."),
            ("mushroom", &["mushroom"], "A pale mushroom. The admiral warned you not to eat it alone."),
            ("champignon", &["champignon"], "A fat champignon, the antidote to the jungle mushroom."),
            ("salt", &["salt"], "Coarse sea salt from the driftwood."),
            ("rosemary", &["rosemary"], "A sprig of wild rosemary."),
            ("oregano", &["oregano"], "A bundle of wild oregano - the poor man's marjoram."),
            ("pepper", &["pepper"], "Black pepper, taken from a sea chest that labelled it gunpowder."),
            ("elixir", &["elixir", "flask"], "A small flask of glowing elixir. It restores an exhausted man."),
            ("eye", &["eye of kal", "eye"], "The Eye of Kal, whole at last: six splinters set in a ring of black metal. It seems to look back at you."),
            ("amulet", &["amulet"], "A sealed bronze amulet. Something small moves inside it."),
            ("ball", &["steel ball", "bridge ball"], "A heavy steel ball. It fits the keyhole of the black door."),
            ("speaker", &["speaker", "loudspeaker"], "A loudspeaker torn from an amplifier. Its magnet is very strong."),
            ("microphone", &["microphone"], "A stage microphone with a long cable."),
            ("wrench", &["wrench", "english"], "An adjustable wrench marked 'ENGLISH'."),
            ("oxygen", &["oxygen"], "An oxygen gear with a charged tank and a mask."),
            ("scale", &["scale"], "A reflective metal scale from the Bool's hide. It throws light like a mirror."),
            ("root", &["root", "vine"], "A long root vine, strong enough to dive with."),
            ("wood", &["piece of wood", "wood"], "A waterlogged piece of wood. A root can be tied to it."),
            ("spider", &["spider"], "A huge but placid monster spider. It sits quietly in your pocket."),
            ("bananas", &["bananas"], "A bunch of ripe bananas."),
            ("meat", &["meat"], "A haunch of meat, still fresh."),
            ("dung", &["dung"], "A clump of dinosaur dung, held in a wrapping of leaves."),
            ("leaves", &["leaves"], "A handful of broad leaves."),
            ("sigma", &["sigma"], "A sigma cell: a small, heavy power cell that hums faintly."),
            ("soot", &["soot"], "A handful of black soot."),
        ];
        for (id, words, text) in checks {
            if self.has(id) && m(n, words) {
                self.print(text);
                return;
            }
        }
        self.print("You see nothing special about that.");
    }

    fn examine_room(&mut self, n: &str) -> bool {
        let room = self.room;
        match room {
            1 => {
                if m(n, &["portrait", "picture", "painting"]) {
                    self.print("The portrait shows a stern man in a black coat: Harold Jugger, the mansion's last owner. Below it hangs a small plaque with a sliding panel.");
                    self.print("On the frame a sheet of music is pinned, but the last note is missing.");
                } else if m(n, &["plaque", "sign", "panel"]) {
                    if self.flag("plaque_open") {
                        self.print("The plaque's secret drawer stands open. It is empty.");
                    } else {
                        self.print("A small bronze plaque under the portrait: 'H. JUGGER - THE EYE SEES ALL.' There is a coin slot and a sliding panel.");
                    }
                } else if m(n, &["chandelier", "lustre"]) {
                    if self.flag("rope_cut") {
                        self.print("The chandelier has come down. The cut rope and a candle lie on the floor.");
                    } else {
                        self.print("A huge chandelier hangs from a rope above the hall. The rope is thick, but a sharp blade would part it.");
                    }
                } else if m(n, &["door", "front door", "entrance"]) {
                    if self.flag("postman") {
                        self.print("The front door stands open. The postman has gone.");
                    } else {
                        self.print("The front door is shut. Someone knocked a moment ago.");
                    }
                } else if m(n, &["note", "zettel"]) {
                    self.print("A scrap of paper by the door, covered in illegible pencil marks. It seems to be a shopping list.");
                } else {
                    return false;
                }
                true
            }
            2 => {
                if m(n, &["sword", "blade", "samurai", "stand"]) {
                    if self.flag("blade_taken") {
                        self.print("The sword stand is empty.");
                    } else {
                        self.print("A fine samurai sword rests on a lacquered stand. The blade is loose in its hilt.");
                    }
                } else if m(n, &["couch", "sofa"]) {
                    if self.flag("couch_moved") {
                        self.print("The couch has been pushed aside. The floor beneath it is polished and bare.");
                    } else {
                        self.print("A heavy leather couch. Something white is caught beneath it, but the couch will not slide on this dull floor.");
                    }
                } else if m(n, &["paper", "note"]) && self.flag("couch_moved") && !self.flag("note_taken") {
                    self.print("A folded sheet of paper lies under the couch. TAKE PAPER to pick it up.");
                } else if m(n, &["book", "shelf", "shelves", "library"]) {
                    self.print("Ranks of leather-bound books. Most are in German, a few in Latin. None seem to hide anything.");
                } else {
                    return false;
                }
                true
            }
            3 => {
                if m(n, &["piano", "keys", "keyboard"]) {
                    if !self.flag("piano_lid") {
                        self.print("A grand piano. Its lid is closed and the keyboard flap is locked.");
                    } else if !self.flag("piano_unlocked") {
                        self.print("The lid is open. The keyboard flap is locked by a tiny keyhole.");
                    } else {
                        self.print("The keyboard is open. The keys are yellowed but sound sweetly.");
                    }
                } else if m(n, &["lighter", "behind", "floor"]) {
                    if self.flag("lighter_taken") {
                        self.print("The hiding place behind the piano is empty now.");
                    } else if self.flag("diary_read") || self.flag("envelope_open") {
                        self.print("Behind the piano, in a hollow in the skirting board, lies a brass lighter.");
                    } else {
                        self.print("Dust and piano legs. You would need a reason to search here.");
                    }
                } else if m(n, &["painting", "landscape"]) {
                    self.print("A romantic landscape: a black tower on a hill, a river of light below it.");
                } else {
                    return false;
                }
                true
            }
            4 => {
                if m(n, &["table", "billiard", "cue", "rack"]) {
                    self.print("A green billiard table with six pockets. Three cues lean in a rack beside it. One of them looks as if it can be unscrewed.");
                } else if m(n, &["pocket", "hole"]) {
                    self.print("Leather pockets. One of them, under the table, has a metal ring set into it.");
                } else if m(n, &["trophy", "shelf"]) {
                    self.print("Silver trophies for billiards, all engraved with the name H. JUGGER.");
                } else {
                    return false;
                }
                true
            }
            5 => {
                if m(n, &["floor", "brick", "stone"]) {
                    if self.flag("brick_open") {
                        self.print("The loose brick has been pressed. A dark stair leads down behind the fireplace.");
                    } else {
                        self.print("Soot-black bricks. One of them is loose and sits slightly proud of the others.");
                    }
                } else if m(n, &["soot"]) {
                    self.print("A handful of dry soot could be taken from the chimney back.");
                } else {
                    return false;
                }
                true
            }
            7 => {
                if m(n, &["bed", "canopy", "towel", "cloth", "plush"]) {
                    self.print("A four-poster bed with a plush canopy cloth. The cloth hangs just out of reach from the floor.");
                } else {
                    return false;
                }
                true
            }
            8 => {
                if m(n, &["bath", "tub", "sponge", "washcloth", "spider"]) {
                    if self.flag("web_destroyed") {
                        self.print("The bath is empty. The spider has retreated into a corner.");
                    } else if self.flag("spider_on_sponge") {
                        self.print("The spider sits on the sponge in the bottom of the bath, watching you.");
                    } else {
                        self.print("A claw-footed bath. A sponge lies in it, and a large spider sits on the sponge.");
                    }
                } else if m(n, &["toilet", "web", "toothbrush", "net"]) {
                    if self.flag("web_destroyed") {
                        self.print("The web is torn away. The toothbrush is yours for the taking.");
                    } else {
                        self.print("A thick web stretches across the toilet. Beneath it lies a toothbrush. The spider would notice anyone reaching through.");
                    }
                } else {
                    return false;
                }
                true
            }
            9 => {
                if m(n, &["table"]) {
                    self.print("A heavy oak table. It can be pushed.");
                } else if m(n, &["ceiling", "trapdoor", "hatch"]) {
                    if self.flag("trapdoor_open") {
                        self.print("The trapdoor stands open. A ladder of cold air leads up to the attic.");
                    } else {
                        self.print("A trapdoor in the ceiling, bolted from below. The bolt is too high to reach from the floor.");
                    }
                } else if m(n, &["bolt", "latch"]) {
                    self.print("An iron bolt, high on the trapdoor.");
                } else {
                    return false;
                }
                true
            }
            10 => {
                if m(n, &["jar", "glass", "marmalade"]) {
                    self.print("A clean jam jar stands on the table.");
                } else if m(n, &["drawer", "wax", "cupboard"]) {
                    self.print("The kitchen drawer holds string, a corkscrew and a tube of floor wax.");
                } else if m(n, &["flypaper", "fly", "flies"]) {
                    self.print("A sticky flypaper strip hangs from the ceiling. It is quite useless - the fly avoids it.");
                } else if m(n, &["table"]) {
                    self.print("A scrubbed kitchen table. It is heavy, but it would slide across a polished floor.");
                } else {
                    return false;
                }
                true
            }
            11 => {
                if m(n, &["bible", "book", "shelf"]) {
                    self.print("A children's bible on the shelf. Its spine is thicker than it should be.");
                } else if m(n, &["toy", "rug", "shelf"]) {
                    self.print("Toy soldiers, a wooden horse and a faded rug. Nothing else of use.");
                } else {
                    return false;
                }
                true
            }
            12 => {
                if m(n, &["crate", "chest", "trunk", "box"]) {
                    self.print("Crates, an old chest and a steamer trunk. The trunk bears stickers from cities you have never visited.");
                } else {
                    return false;
                }
                true
            }
            13 => {
                if m(n, &["junk", "tools", "garden", "hoe", "hook"]) {
                    self.print("Rusty garden tools, a broken hoe and a coil of wire. A strong iron hook lies among them.");
                } else if m(n, &["buffet", "cupboard", "drawer", "sugar"]) {
                    if self.flag("buffet_open") {
                        self.print("The buffet drawer is open. A few crumbs of sugar remain.");
                    } else {
                        self.print("An old kitchen buffet. One drawer is stuck but not locked.");
                    }
                } else {
                    return false;
                }
                true
            }
            14 => {
                if m(n, &["bottle", "wine", "label", "rack"]) {
                    self.print("Dusty bottles in wooden racks. Each label carries a year. One rack is labelled 1922.");
                } else {
                    return false;
                }
                true
            }
            16 => {
                if m(n, &["block", "stone", "quader"]) {
                    self.print("A great stone block. A faint draft comes from behind it.");
                } else if m(n, &["floor", "ground", "pickaxe"]) {
                    self.print("The floor sounds hollow near the block. A pickaxe leans against the wall.");
                } else {
                    return false;
                }
                true
            }
            24 => {
                if m(n, &["slot", "cryptogram", "carving", "wall"]) {
                    self.print("The walls are carved with cryptograms. Beside a broken lever is a square slot exactly the size of a wooden handle.");
                } else if m(n, &["lever", "handle"]) {
                    if self.flag("lever_broken") {
                        self.print("The old lever has snapped off. The slot waits for a replacement.");
                    } else {
                        self.print("A stone lever, already cracked at its base.");
                    }
                } else if m(n, &["face", "monster", "maw"]) {
                    self.print("A monstrous face is carved above the passage. Its mouth is a black hole.");
                } else {
                    return false;
                }
                true
            }
            26 => {
                if m(n, &["field", "fields", "colour", "color", "floor"]) {
                    self.print("Six coloured fields surround the blocks: white, yellow, green, red, blue and black.");
                } else if m(n, &["block", "stone", "numeral", "quader"]) {
                    self.print("Six stone blocks, each marked with a Roman numeral: I to VI.");
                } else if m(n, &["poem", "diary", "journal"]) {
                    self.print("The diary's poem links each world to a colour and a numeral. Type JOURNAL to read it.");
                } else {
                    return false;
                }
                true
            }
            _ => false,
        }
    }

    fn examine_later(&mut self, n: &str) -> bool {
        let room = self.room;
        let _ = match room {
            27 => {
                if m(n, &["crate", "box"]) {
                    self.print("A wooden crate, its lid nailed shut but not locked.");
                } else if m(n, &["barrel", "cask", "oil", "tran"]) {
                    self.print("A barrel of whale oil. The surface glistens.");
                } else if m(n, &["boulder", "rock", "goggles", "snow"]) {
                    self.print("A snow-covered boulder. A pair of snow goggles lies on top.");
                } else if m(n, &["flag", "club"]) {
                    self.print("A flag on a pole and a heavy wooden club. Both could be useful.");
                } else {
                    return false;
                }
                true
            }
            32 => {
                if m(n, &["monolith", "relief", "face", "socket", "eye"]) {
                    self.print("A monolith of black stone. Reliefs show six worlds. A monstrous face has one empty eye socket, and the stone around it is warm.");
                } else {
                    return false;
                }
                true
            }
            33 => {
                if m(n, &["shaft", "crater", "debris", "hole"]) {
                    self.print("A deep shaft into darkness. Hot debris steams around the rim. The drop is far too great to survive.");
                } else {
                    return false;
                }
                true
            }
            35 => {
                if m(n, &["sign", "schild"]) {
                    self.print("A sign frozen to the rock: 'ICE CAVE - KEEP OUT. BEARS.'");
                } else if m(n, &["gate", "grille", "door"]) {
                    self.print("A barred iron gate. The lock is large and old.");
                } else {
                    return false;
                }
                true
            }
            36 => {
                if m(n, &["shrine", "shard", "altar"]) {
                    self.print("A shrine of ice. A gem splinter glitters on it.");
                } else {
                    return false;
                }
                true
            }
            65 => {
                if m(n, &["officer", "captain"]) {
                    self.print("An English officer scans the horizon through a glass. He does not answer.");
                } else if m(n, &["flag", "cannon", "plank", "deck"]) {
                    self.print("The deck is scrubbed, the cannons are lashed, and the flag snaps in a rising wind.");
                } else {
                    return false;
                }
                true
            }
            66 => {
                if m(n, &["willy", "ochsen"]) {
                    self.print("Ochsen-Willy, a broad man with quick hands. He keeps looking at your pockets.");
                } else if m(n, &["pete", "pocken"]) {
                    self.print("Pocken-Pete. His face is scarred and one leg is badly hurt - he can only limp.");
                } else if m(n, &["cannonball", "cannon ball", "ball", "kugel"]) {
                    if !self.flag("rammer_offered") {
                        self.set("rammer_offered");
                        self.print("You heft a round shot. Willy grins and holds out a powder rammer. TAKE RAMMER.");
                    } else {
                        self.print("A rack of round shot for the cannon.");
                    }
                } else if m(n, &["cannon", "powder", "rammer"]) {
                    self.print("A cannon, a rack of round shot, and a powder charge waiting to be rammed home.");
                } else {
                    return false;
                }
                true
            }
            69 => {
                if m(n, &["plank", "wood"]) {
                    self.print("A broad plank, just buoyant enough. Several knot holes dot its surface. The middle one is larger than the rest.");
                } else if m(n, &["knot", "hole"]) {
                    self.print("Knot holes. In the middle hole something pale is jammed tight - a broken twig.");
                } else {
                    return false;
                }
                true
            }
            70 => {
                if m(n, &["rock", "cone", "cliff"]) {
                    self.print("A cone of volcanic rock. The jungle presses close around it.");
                } else if m(n, &["palm", "coconut", "tree"]) {
                    self.print("A coconut palm leans over the sand. A coconut hangs high in its crown.");
                } else if m(n, &["plank", "wood", "salt", "driftwood"]) {
                    self.print("The ship's plank has washed ashore. Salt crystals crust its wood.");
                } else if m(n, &["jungle", "thicket", "shrub", "rosemary"]) {
                    self.print("Dense green growth. Among the shrubs a sprig of rosemary grows.");
                } else {
                    return false;
                }
                true
            }
            73 => {
                if m(n, &["willy", "body", "corpse"]) {
                    self.print("Willy lies behind the rock, drowned. His pockets are empty. He deserves a grave.");
                } else {
                    return false;
                }
                true
            }
            74 => {
                if m(n, &["chest", "sea chest", "water", "bay"]) {
                    self.print("A sea chest floats out in the bay, just beyond reach of your hands.");
                } else {
                    return false;
                }
                true
            }
            76 => {
                if m(n, &["paper", "warning", "note"]) {
                    self.print("A warning to Admiral Atterson: 'Do not enter the jungle. The blossoms are not what they seem.'");
                } else if m(n, &["stone", "bottle", "flask"]) {
                    self.print("A smooth stone lies on the paper. A bottle stands beside it, smelling of Maggi seasoning.");
                } else {
                    return false;
                }
                true
            }
            81 => {
                if m(n, &["floor", "boards", "wood"]) {
                    self.print("Loose wooden boards. Something glints between them.");
                } else {
                    return false;
                }
                true
            }
            83 => {
                if m(n, &["grass", "carpet", "mushroom"]) {
                    self.print("A carpet of grass. Pale mushrooms grow in a ring.");
                } else if m(n, &["shrub", "bush", "oregano"]) {
                    self.print("A tangle of shrubs. One of them smells strongly of oregano.");
                } else {
                    return false;
                }
                true
            }
            87 => {
                if m(n, &["winch", "crank", "peg", "knob"]) {
                    self.print("A rope winch with a wooden crank. A peg sticks out sideways, just asking to be knocked loose.");
                } else if m(n, &["bridge", "chasm"]) {
                    self.print("A wooden bridge, raised on the far side of the chasm.");
                } else {
                    return false;
                }
                true
            }
            88 => {
                if m(n, &["door", "rock door", "wall"]) {
                    self.print("A door of fitted rock, shut fast and without a handle.");
                } else {
                    return false;
                }
                true
            }
            95 => {
                if m(n, &["pete", "body", "wound", "clothes", "eyepatch"]) {
                    self.print("Pete's body. His leg wound is obvious, and he still wears his clothes and eyepatch. A disguise might fool something that only knows him by sight and gait.");
                } else {
                    return false;
                }
                true
            }
            96 => {
                if m(n, &["shard", "needle", "monster", "tentacle"]) {
                    self.print("A gem splinter rests on a rock needle. A tentacled monster guards it and demands Kal's sign.");
                } else {
                    return false;
                }
                true
            }
            120 => {
                if m(n, &["thicket", "bush", "undergrowth"]) {
                    self.print("Dense undergrowth. Something metal glints inside it.");
                } else if m(n, &["barrier", "barriers", "sign", "paint", "number"]) {
                    self.print("Red and white barriers. A painted sign gives the site number: 7.");
                } else if m(n, &["hovercraft", "vehicle", "skimmer"]) {
                    self.print("A hovercraft on its cushion, lashed to the site. The cockpit canopy is open.");
                } else {
                    return false;
                }
                true
            }
            121 => {
                if m(n, &["sticker", "advert", "sign"]) {
                    self.print("A sticker: 'VISIT THE HISTORICAL MUSEUM, BALTIMORE - RELICS OF LOST CULTURES.'");
                } else if m(n, &["panel", "dashboard", "instrument", "diagram"]) {
                    self.print("The instrument panel has a switch, a knob and an ignition. A diagram shows the hover circuit.");
                } else if m(n, &["seat", "compartment", "tool"]) {
                    self.print("The seat lifts to reveal a tool compartment.");
                } else if m(n, &["key", "ignition"]) {
                    self.print("The ignition waits for a key.");
                } else {
                    return false;
                }
                true
            }
            122 => {
                if m(n, &["cover", "plate", "cladding", "chassis"]) {
                    self.print("A metal cover plate held by a single recessed screw.");
                } else if m(n, &["unit", "box", "plastic", "maintenance"]) {
                    self.print("The maintenance unit. Its plastic box is cracked but still sealed.");
                } else if m(n, &["cable", "wire", "drum", "thread", "plug"]) {
                    self.print("A cable drum. A heavy plug hangs from it on a threaded collar.");
                } else {
                    return false;
                }
                true
            }
            125 => {
                if m(n, &["vehicle", "repair", "machine"]) {
                    self.print("A gigantic repair vehicle: a ladder to the tunnel, a robot arm, and a driver's stand.");
                } else if m(n, &["ladder", "tunnel"]) {
                    self.print("A service ladder climbs into the machine's tunnel.");
                } else if m(n, &["arm", "robot", "claw"]) {
                    self.print("A jointed robot arm ending in a claw. It is currently lifeless.");
                } else {
                    return false;
                }
                true
            }
            126 => {
                if m(n, &["lever", "button", "switch", "dead man"]) {
                    self.print("A control lever, a power button and a dead-man switch labelled in English.");
                } else {
                    return false;
                }
                true
            }
            127 => {
                if m(n, &["tongs", "platform"]) {
                    if self.flag("tongs_fallen") {
                        self.print("A pair of tongs lies on the platform.");
                    } else {
                        self.print("A pair of tongs rests on a high ledge, just out of reach.");
                    }
                } else {
                    return false;
                }
                true
            }
            128 => {
                if m(n, &["door", "intercom", "speaker", "button"]) {
                    self.print("A service door with an intercom grille and a single button.");
                } else {
                    return false;
                }
                true
            }
            134 => {
                if m(n, &["robo", "robot", "cleaner"]) {
                    self.print("A cleaning robot with a single camera eye. It waits by the turnstile, humming.");
                } else if m(n, &["srt", "train", "platform"]) {
                    self.print("An SRT car stands at the platform, its door closed.");
                } else {
                    return false;
                }
                true
            }
            135 => {
                if m(n, &["panel", "buttons", "button"]) {
                    self.print("A panel of numbered destination buttons. The door control is beside it.");
                } else {
                    return false;
                }
                true
            }
            136 => {
                if m(n, &["slot", "lift", "door"]) {
                    self.print("A card slot glows beside the lift door.");
                } else {
                    return false;
                }
                true
            }
            137 => {
                if m(n, &["terminal", "computer"]) {
                    self.print("An active terminal. USE TERMINAL to begin an information query.");
                } else {
                    return false;
                }
                true
            }
            139 => {
                if m(n, &["fountain", "water", "nozzle", "basin"]) {
                    self.print("A fountain, dry. Copper nozzles ring its basin, and a control box is set into the base.");
                } else if m(n, &["roboss", "robot", "guard"]) {
                    self.print("A three-eyed Roboss guards the vending machine to the west. Water would play havoc with its electrics.");
                } else if m(n, &["vending", "machine", "automat"]) {
                    self.print("A vending machine with a button and a delivery slot. The Roboss blocks the way.");
                } else if m(n, &["barrier", "barriers", "west", "quicksand"]) {
                    self.print("A building barrier. Dry quicksand has collected in its shadow.");
                } else {
                    return false;
                }
                true
            }
            140 => {
                if m(n, &["machine", "vending", "button", "slot"]) {
                    self.print("The vending machine's button and slot are within reach now.");
                } else {
                    return false;
                }
                true
            }
            143 | 144 => {
                if m(n, &["scene", "display", "case", "picture"]) {
                    self.print("The museum scenes show six worlds: ice, a new world, the sea, the dawn of man, ships, and a mine of fire. One display is a ruined temple; another shows a black city under a red sun.");
                } else {
                    return false;
                }
                true
            }
            145 => {
                if m(n, &["terminal", "slot", "button"]) {
                    self.print("A museum terminal with a button and an output slot.");
                } else {
                    return false;
                }
                true
            }
            146 => {
                if m(n, &["dung", "heap", "fladen"]) {
                    self.print("A steaming heap of dinosaur dung. Ripe, and too foul to pick up with bare hands.");
                } else if m(n, &["tyrannosaurus", "rex", "dinosaur"]) {
                    self.print("A tyrannosaurus towers over the way north, jaws working.");
                } else {
                    return false;
                }
                true
            }
            147 => {
                if m(n, &["footprint", "print", "mud", "fossil"]) {
                    self.print("Huge three-toed footprints. Broad leaves are caught between them.");
                } else {
                    return false;
                }
                true
            }
            151 => {
                if m(n, &["tiger", "sabertooth", "meat", "brontosaurus"]) {
                    self.print("A sabertooth tiger gnaws a haunch of meat. A brontosaurus grazes to the west.");
                } else {
                    return false;
                }
                true
            }
            153 => {
                if m(n, &["baby", "dinosaur", "shard", "rock"]) {
                    self.print("A baby dinosaur sits on the rock, clutching a gem splinter.");
                } else {
                    return false;
                }
                true
            }
            156 => {
                if m(n, &["sandbank", "shrub", "findling", "rock", "trunk"]) {
                    self.print("A sandbank across the water. A fallen tree trunk reaches toward it, and a huge spider squats on the sand.");
                } else {
                    return false;
                }
                true
            }
            157 => {
                if m(n, &["spider", "banana", "palm"]) {
                    self.print("A monstrous spider, quite calm, beside a banana palm.");
                } else {
                    return false;
                }
                true
            }
            160 => {
                if m(n, &["man", "slave", "worker"]) {
                    self.print("An exhausted man lies at the south end of the mine. He is still breathing.");
                } else if m(n, &["cart", "chassis", "mechanism", "lever", "rails"]) {
                    self.print("A mine cart on rails. Its chassis has a lever and a place to tie a rope.");
                } else {
                    return false;
                }
                true
            }
            165 => {
                if m(n, &["station", "rails", "stones", "ore"]) {
                    self.print("The great ore station. Rails cross in a maze, and ore is heaped between them.");
                } else {
                    return false;
                }
                true
            }
            169 => {
                if m(n, &["man", "stranger", "tell"]) {
                    self.print("A weathered man watches you from beside the rails.");
                } else {
                    return false;
                }
                true
            }
            170 => {
                if m(n, &["forge", "anvil", "furnace", "smith", "man"]) {
                    self.print("A volcano forge. The smith stands at an anvil beside a furnace that breathes orange light.");
                } else {
                    return false;
                }
                true
            }
            174 => {
                if m(n, &["amulet", "smith", "man", "pit"]) {
                    self.print("Tell the smith waits by the tunnel. He holds out a bronze amulet and says something about a kobold named Pit.");
                } else {
                    return false;
                }
                true
            }
            175 => {
                if m(n, &["ball", "hollow", "bridge"]) {
                    self.print("A steel ball rests in a hollow in the bridge. Taking it might disturb the span.");
                } else if m(n, &["beast", "bestie"]) {
                    self.print("The beast crouches beyond the bridge, speaking in a voice too loud for its size.");
                } else {
                    return false;
                }
                true
            }
            177 => {
                if m(n, &["beast", "crack", "ceiling", "cage"]) {
                    self.print("The beast points at a crack in the floor, then at the ceiling. Both suggestions smell like traps.");
                } else {
                    return false;
                }
                true
            }
            178 => {
                if m(n, &["steps", "stairs", "microphone", "cable", "box", "speaker"]) {
                    self.print("Worn steps. A microphone lies on one of them, its cable running down to an amplifier box whose woofers are missing a speaker.");
                } else if m(n, &["cage", "ceiling"]) {
                    self.print("An iron cage hangs from the ceiling above the stairs.");
                } else {
                    return false;
                }
                true
            }
            179 => {
                if m(n, &["keyhole", "door", "lock"]) {
                    self.print("A great black door with a round keyhole, large enough for a steel ball.");
                } else {
                    return false;
                }
                true
            }
            181 => {
                if m(n, &["paper", "lectern", "crypt"]) {
                    self.print("A paper on the lectern: 'Every chamber of the labyrinth has six ways out. Only the riddles of Pit lead through.'");
                } else {
                    return false;
                }
                true
            }
            184 => {
                if m(n, &["lake", "water", "bool", "monster"]) {
                    self.print("An underground lake, black and still. Something enormous moves below the surface.");
                } else {
                    return false;
                }
                true
            }
            190 => {
                if m(n, &["sand", "wall", "plant", "stonewort", "light"]) {
                    self.print("Sand, a rock wall with a plant clinging to it, and a beam of light falling from below.");
                } else {
                    return false;
                }
                true
            }
            191 => {
                if m(n, &["bool", "monster", "mouth", "scale", "teeth"]) {
                    self.print("The dead Bool lies on the lake bed: glass and plastic, with a mouth full of sharp teeth and scales held by bolts.");
                } else {
                    return false;
                }
                true
            }
            192 => {
                if m(n, &["panel", "button", "switch", "cladding", "wrench"]) {
                    self.print("A control panel with a mouth button and a switch. Metal cladding covers a compartment.");
                } else {
                    return false;
                }
                true
            }
            193 | 194 => {
                if m(n, &["mesh", "bolt", "skin", "scale"]) {
                    self.print("A metal mesh of bolts holds one great scale in place.");
                } else {
                    return false;
                }
                true
            }
            195 => {
                if m(n, &["sluice", "gate", "grille", "control", "regulator", "button", "key"]) {
                    self.print("The sluice control: a button for the grille, a key for the steel gate, and a regulator for the water.");
                } else if m(n, &["cable", "wire"]) {
                    if self.flag("cable195_cut") {
                        self.print("The control cable is severed.");
                    } else {
                        self.print("A thick cable feeds the control unit.");
                    }
                } else {
                    return false;
                }
                true
            }
            196 => {
                if m(n, &["water", "basin", "monster", "bool"]) {
                    self.print("A flooded basin. A second Bool moves in the water.");
                } else {
                    return false;
                }
                true
            }
            198 => {
                if m(n, &["chair", "throne", "giant", "drawbar", "lever", "cable", "torch"]) {
                    self.print("Kathaura's watch: a giant's chair, torches, a drawbar and a control cable. The way north is barred.");
                } else {
                    return false;
                }
                true
            }
            199 => {
                if m(n, &["door", "gate", "button"]) {
                    self.print("A great door on the black way. It has neither handle nor keyhole, only a seam where a lever could be forced in.");
                } else {
                    return false;
                }
                true
            }
            200 => {
                if m(n, &["spell", "writing", "wall", "inscription"]) {
                    self.print("A spell is cut into the wall: 'To walk unseen in the black city, take the colour of the stone into your skin.'");
                } else if m(n, &["chair", "seat", "throne"]) {
                    self.print("A giant chair. It can be pushed, but not carried.");
                } else if m(n, &["soot", "black"]) {
                    self.print("Soot coats the wall behind the chair. It would blacken a man from head to foot.");
                } else {
                    return false;
                }
                true
            }
            202 => {
                if m(n, &["khalin", "giantess", "window", "glasses", "button"]) {
                    self.print("Beyond a window, Khalin the giantess peers down at the city. Her glasses are enormous. A button waits beside the gate.");
                } else {
                    return false;
                }
                true
            }
            205 => {
                if m(n, &["khalin", "glass", "glasses", "balustrade", "chair", "bench"]) {
                    self.print("Khalin's pavilion on the giant table. A chair and a bench stand by the balustrade. Khalin may set her glasses down if you wait.");
                } else {
                    return false;
                }
                true
            }
            233 => {
                if m(n, &["floor", "stone", "plate", "slab"]) {
                    self.print("A stone floor. One slab sits slightly proud of the rest.");
                } else {
                    return false;
                }
                true
            }
            234 | 235 => {
                if m(n, &["machine", "apparatus", "key", "switch", "cannon"]) {
                    self.print("Machines and apparatus crowd the chamber. A panel with a switch waits on the south wall, and a key hangs in the north passage.");
                } else {
                    return false;
                }
                true
            }
            236 => {
                if m(n, &["glass", "wall", "man", "kal", "nameplate", "button"]) {
                    self.print("A glass wall. Behind it a man sits with his back turned. A nameplate reads KAL. A button is set into the frame.");
                } else {
                    return false;
                }
                true
            }
            _ => return false,
        };
        true
    }

    // ------------------------------------------------------------------
    // Taking and putting down
    // ------------------------------------------------------------------

    fn pick_up(&mut self, n: &str) {
        // Loose objects that are not tied to one room.
        if m(n, &["shard", "splitter", "gem", "edelstein"]) && self.flag("envelope_open") && !self.flag("took_shard1") {
            self.set("took_shard1");
            self.award(50);
            self.print("You take the first gem splinter. It is cold and bright, and it seems to weigh more than it should.");
            return;
        }
        if m(n, &["fly", "fliege"]) && self.has("jar") && self.flag("fly_dead") && !self.has("fly") {
            self.add("fly");
            self.set("fly_taken");
            self.print("You shake the dead fly out of the jar and keep it.");
            return;
        }
        let room = self.room;
        let mut done = true;
        match room {
            1 => {
                if m(n, &["envelope", "letter", "post"]) && self.flag("postman") && !self.flag("envelope_taken") {
                    self.add("envelope");
                    self.set("envelope_taken");
                    self.print("You take the envelope. It is heavy and tied with cord.");
                } else if m(n, &["candle"]) && self.flag("rope_cut") && !self.has("candle") {
                    self.add("candle");
                    self.print("You pick up the fallen candle.");
                } else if m(n, &["rope", "cord"]) && self.flag("rope_cut") && !self.has("rope") {
                    self.add("rope");
                    self.print("You coil the cut chandelier rope.");
                } else {
                    done = false;
                }
            }
            2 => {
                if m(n, &["sword", "blade", "samurai"]) && !self.flag("blade_taken") {
                    self.add("blade");
                    self.set("blade_taken");
                    self.print("You lift the samurai sword from its stand. The blade slides free of the old hilt - you have a sword blade.");
                } else if m(n, &["paper", "note"]) && self.flag("couch_moved") && !self.flag("note_taken") {
                    self.add("note");
                    self.set("note_taken");
                    self.award(25);
                    self.print("You fish the paper out from under the couch.");
                } else {
                    done = false;
                }
            }
            3 => {
                if m(n, &["lighter", "feuerzeug"]) && !self.flag("lighter_taken") {
                    if self.flag("envelope_open") || self.flag("diary_read") {
                        self.add("lighter");
                        self.set("lighter_taken");
                        self.print("You reach behind the piano and take the brass lighter.");
                    } else {
                        self.print("You don't know to look behind the piano yet.");
                    }
                } else {
                    done = false;
                }
            }
            4 => {
                if m(n, &["cue", "stick", "billiard"]) && !self.has("cue") && !self.has("cue_shaft") {
                    self.add("cue");
                    self.print("You take a billiard cue from the rack. It is the third one - and it unscrews.");
                } else {
                    done = false;
                }
            }
            5 => {
                if m(n, &["soot", "russ"]) && !self.has("soot") {
                    self.add("soot");
                    self.print("You scrape up a handful of soot.");
                } else {
                    done = false;
                }
            }
            7 => {
                if m(n, &["cloth", "plush", "towel"]) && self.flag("bed_jumped") && !self.flag("cloth_taken") {
                    self.add("cloth");
                    self.set("cloth_taken");
                    self.award(10);
                    self.print("You pull the plush cloth down from the canopy.");
                } else if m(n, &["cloth", "plush"]) {
                    self.print("The cloth is too high. Perhaps the bed would help.");
                } else {
                    done = false;
                }
            }
            8 => {
                if m(n, &["sponge", "washcloth"]) && !self.has("sponge") && (self.flag("fly_given") || self.flag("spider_on_sponge") || self.flag("web_destroyed")) {
                    self.add("sponge");
                    self.print("You take the sponge from the bath.");
                } else if m(n, &["toothbrush", "buerste", "bürste"]) && self.flag("web_destroyed") && !self.has("toothbrush") {
                    self.add("toothbrush");
                    self.award(10);
                    self.print("You take the toothbrush from under the torn web.");
                } else if m(n, &["sponge", "washcloth"]) {
                    self.print("The spider is sitting on the sponge. You need to distract it first.");
                } else {
                    done = false;
                }
            }
            9 => {
                if m(n, &["table"]) {
                    self.print("The table is too heavy to carry, but it can be pushed.");
                } else {
                    done = false;
                }
            }
            10 => {
                if m(n, &["jar", "glass"]) && !self.has("jar") {
                    self.add("jar");
                    self.print("You take the jam jar.");
                } else if m(n, &["wax", "tube"]) && !self.has("wax") {
                    self.add("wax");
                    self.print("You take the tube of floor wax from the drawer.");
                } else {
                    done = false;
                }
            }
            11 => {
                if m(n, &["bible"]) && !self.has("bible") && !self.flag("coin_taken") {
                    self.add("bible");
                    self.print("You take the children's bible. Something rattles inside its spine.");
                } else if m(n, &["coin", "muenze", "münze"]) && self.flag("bible_open") && !self.flag("coin_taken") {
                    self.add("coin");
                    self.set("coin_taken");
                    self.print("You work the silver coin out of the bible's spine.");
                } else {
                    done = false;
                }
            }
            13 => {
                if m(n, &["hook", "haken"]) && !self.has("hook") {
                    self.add("hook");
                    self.print("You take the iron hook from the garden tools.");
                } else if m(n, &["sugar", "wuerfel", "würfel"]) && self.flag("buffet_open") && !self.has("sugar") && !self.flag("sugar_in_jar") {
                    self.add("sugar");
                    self.print("You take the sugar cube from the buffet drawer.");
                } else {
                    done = false;
                }
            }
            14 => {
                if m(n, &["bottle", "1922", "wine", "petroleum"]) && !self.flag("bottle_taken") {
                    self.add("bottle");
                    self.set("bottle_taken");
                    self.print("You take the bottle labelled 1922.");
                } else {
                    done = false;
                }
            }
            15 => {
                if m(n, &["hook", "haken"]) && !self.has("hook") {
                    self.add("hook");
                    self.print("You take the iron hook from among the garden tools.");
                } else if m(n, &["pickaxe", "pick", "hacke"]) && !self.has("pickaxe") {
                    self.add("pickaxe");
                    self.print("You take the garden pickaxe.");
                } else if m(n, &["junk", "rubbish", "tools", "geruempel", "gerümpel"]) {
                    self.print("Broken chairs, crates and rusty garden tools. The hook and the pickaxe look useful.");
                } else {
                    done = false;
                }
            }
            16 => {
                if m(n, &["pickaxe", "pick", "hacke"]) && !self.has("pickaxe") {
                    self.add("pickaxe");
                    self.print("You take the pickaxe.");
                } else if m(n, &["lamp", "funzel"]) {
                    self.print("A dim little lamp. It is worthless; leave it.");
                } else {
                    done = false;
                }
            }
            27 => {
                if m(n, &["club", "knueppel", "knüppel"]) && !self.has("club") && !self.has("torch") {
                    self.add("club");
                    self.print("You take the wooden club.");
                } else if m(n, &["flag", "fahne"]) && !self.has("flag") && !self.has("torch") {
                    self.add("flag");
                    self.print("You take the flag.");
                } else if m(n, &["boots", "stiefel"]) && self.flag("crate_open") && !self.has("boots") {
                    self.add("boots");
                    self.print("You take the fur boots from the crate.");
                } else if m(n, &["coat", "mantel"]) && self.flag("crate_open") && !self.has("coat") {
                    self.add("coat");
                    self.set("coat_searched");
                    self.print("You take the fur coat. Its pockets hold lined gloves, and a hood is folded at the collar.");
                } else if m(n, &["gloves", "handschuhe"]) && self.flag("coat_searched") && !self.has("gloves") {
                    self.add("gloves");
                    self.print("You take the gloves from the coat pocket.");
                } else if m(n, &["hood", "kapuze"]) && self.flag("coat_searched") && !self.has("hood") {
                    self.add("hood");
                    self.print("You unfold the fur hood.");
                } else if m(n, &["goggles", "brille"]) && !self.has("goggles") {
                    self.add("goggles");
                    self.print("You take the snow goggles from the boulder.");
                } else if m(n, &["snow", "schnee"]) {
                    self.add("snow");
                    self.print("You scoop up a handful of snow.");
                } else {
                    done = false;
                }
            }
            28 | 32 | 33 => {
                if self.room != 33 && m(n, &["snow", "schnee"]) {
                    self.add("snow");
                    self.print("You scoop up a handful of snow.");
                } else if (self.room == 32 || self.room == 33) && m(n, &["shard", "splitter", "gem"]) && self.flag("monolith_gone") && !self.flag("took_crater_shard") {
                    self.give_shard(1);
                    self.set("took_crater_shard");
                    self.print("You take the gem splinter back out of the hot crater.");
                } else {
                    done = false;
                }
            }
            34 => {
                if m(n, &["key", "schluessel", "schlüssel"]) && self.flag("snow_pile_searched") && !self.has("icekey") {
                    self.add("icekey");
                    self.award(10);
                    self.print("You dig the iron key out of the snow.");
                } else {
                    done = false;
                }
            }
            36 => {
                if m(n, &["shard", "splitter", "gem"]) {
                    if self.flag("bear_gone") {
                        self.give_shard(3);
                        self.print("You take the third gem splinter from the shrine.");
                        self.award(50);
                        self.enter(26);
                        self.print("The icy world dissolves, and you are back in the secret cellar.");
                    } else {
                        self.print("The ice bear stands between you and the shrine.");
                    }
                } else {
                    done = false;
                }
            }
            66 => {
                if m(n, &["rammer", "stampfer"]) && self.flag("rammer_offered") && !self.has("rammer") {
                    self.add("rammer");
                    self.print("Willy holds out a powder rammer. You take it.");
                } else if m(n, &["powder", "pulver"]) && self.flag("french_arrived") && !self.has("powder") {
                    self.add("powder");
                    self.print("You snatch the powder charge before the gun crew scatters.");
                } else {
                    done = false;
                }
            }
            77 => {
                if m(n, &["coconut", "kokos"]) && self.flag("thorns_taken") && !self.has("coconut") {
                    self.add("coconut");
                    self.print("You pick the coconut up again.");
                } else {
                    done = false;
                }
            }
            91 => {
                if m(n, &["shard", "splitter", "gem"]) && !self.flag("shard4") {
                    self.give_shard(4);
                    self.award(50);
                    self.print("You take the fourth gem splinter from the chest.");
                    self.enter(26);
                    self.print("The island dissolves, and you are back in the secret cellar.");
                } else {
                    done = false;
                }
            }
            69 => {
                if m(n, &["twig", "branch", "ast"]) && !self.has("twig") && !self.flag("twig_taken") {
                    self.add("twig");
                    self.set("twig_taken");
                    self.award(10);
                    self.print("You work the broken twig out of the middle knot hole.");
                } else if m(n, &["pole", "mast", "stange"]) && self.flag("at_pole") && !self.has("pole") {
                    self.add("pole");
                    self.print("You take the floating wooden pole.");
                } else if m(n, &["sail", "segel"]) && self.flag("at_sail") && !self.has("sail") {
                    self.add("sail");
                    self.print("You haul the sailcloth out of the water.");
                } else {
                    done = false;
                }
            }
            70 => {
                if m(n, &["salt", "salz"]) && !self.has("salt") {
                    self.add("salt");
                    self.print("You scrape sea salt from the plank.");
                } else if m(n, &["rosemary", "rosmarin"]) && !self.has("rosemary") {
                    self.add("rosemary");
                    self.print("You pick a sprig of rosemary.");
                } else {
                    done = false;
                }
            }
            76 => {
                if m(n, &["paper", "warning"]) && !self.flag("warning_read") {
                    self.add("paper");
                    self.set("warning_read");
                    self.print("You take the warning note.");
                } else if m(n, &["stone", "stein"]) && !self.has("stone") && !self.flag("stone_taken") {
                    self.add("stone");
                    self.set("stone_taken");
                    self.award(10);
                    self.print("You take the stone that held the paper down.");
                } else if m(n, &["bottle", "flask", "maggi"]) && !self.has("bottle_maggi") {
                    self.add("bottle_maggi");
                    self.print("You take the bottle of Maggi seasoning.");
                } else {
                    done = false;
                }
            }
            81 => {
                if m(n, &["cork", "kronenkorken"]) && !self.has("cork") {
                    self.add("cork");
                    self.print("You pick the crown cork out of the boards.");
                } else {
                    done = false;
                }
            }
            83 => {
                if m(n, &["mushroom", "pilz"]) && !self.has("mushroom") && !self.flag("mushroom_eaten") {
                    self.add("mushroom");
                    self.print("You take a pale mushroom. You remember the admiral's warning not to eat it alone.");
                } else if m(n, &["oregano"]) && !self.has("oregano") {
                    self.add("oregano");
                    self.print("You pick a bundle of wild oregano.");
                } else {
                    done = false;
                }
            }
            88 => {
                if m(n, &["ball", "kugel"]) {
                    self.print("The steel ball is on the bridge, not here.");
                } else {
                    done = false;
                }
            }
            95 => {
                if m(n, &["clothes", "kleider", "coat", "jacket"]) && !self.flag("pete_clothes") {
                    self.set("pete_clothes");
                    self.print("You strip Pete's salt-stained jacket and trousers. They are your size.");
                } else if m(n, &["eyepatch", "augenklappe"]) && !self.flag("pete_patch") {
                    self.set("pete_patch");
                    self.print("You take Pete's eyepatch.");
                } else {
                    done = false;
                }
            }
            96 => {
                if m(n, &["shard", "splitter", "gem"]) {
                    if self.flag("pete_disguise") {
                        self.give_shard(1);
                        self.print("You take the gem splinter. It is the one Willy stole from you.");
                        self.award(50);
                        self.set("shard_recovered");
                        self.enter(88);
                        self.print("You hurry back to the bridge.");
                    } else if self.flag("pete_clothes") && self.flag("pete_patch") && self.flag("palm_scratched") {
                        self.print("The clothes and eyepatch are not enough. You must also move like Pete - LIMP NORTH.");
                    } else {
                        self.print("A tentacle lashes out. You need Kal's sign, or a convincing disguise.");
                    }
                } else {
                    done = false;
                }
            }
            120 => {
                if m(n, &["tin", "dose", "can"]) && !self.has("tin") && !self.flag("tin_opened") {
                    self.add("tin");
                    self.print("You pull a dented breakfast tin out of the thicket.");
                } else if m(n, &["key", "schluessel"]) && self.flag("tin_opened") && !self.flag("car_key_taken") {
                    self.add("car_key");
                    self.set("car_key_taken");
                    self.print("You take the little ignition key from the tin.");
                    self.award(10);
                } else {
                    done = false;
                }
            }
            121 => {
                if m(n, &["hammer", "crowbar", "compartment", "tool"]) && self.flag("compartment_open") && !self.flag("tool_taken") {
                    self.add("hammer");
                    self.add("crowbar");
                    self.set("tool_taken");
                    self.print("You take a hammer and a crowbar from the tool compartment.");
                } else if m(n, &["passkey", "key"]) && self.flag("compartment_open") && !self.has("passkey") {
                    self.add("passkey");
                    self.print("You look again and find a passkey wedged in the corner of the compartment.");
                } else {
                    done = false;
                }
            }
            122 => {
                if m(n, &["fuse", "sicherung"]) && self.flag("plastic_broken") && !self.has("fuse") {
                    self.add("fuse");
                    self.print("You take the fuse from the broken maintenance unit.");
                } else if m(n, &["plug", "stecker"]) && !self.has("plug") && !self.flag("plug_attached") {
                    self.add("plug");
                    self.print("You take the heavy electric plug.");
                    self.award(10);
                } else {
                    done = false;
                }
            }
            127 => {
                if m(n, &["tongs", "zange"]) && self.flag("tongs_fallen") && !self.has("tongs") {
                    self.add("tongs");
                    self.print("You pick up the tongs from the platform.");
                } else {
                    done = false;
                }
            }
            139 => {
                if m(n, &["quicksand", "sand"]) && !self.has("quicksand") {
                    self.add("quicksand");
                    self.print("You scoop up a handful of dry quicksand.");
                } else if m(n, &["bucket", "eimer"]) && (self.flag("bucket_free") || self.flag("bucket_full")) && !self.has("bucket") {
                    self.add("bucket");
                    self.unset("bucket_free");
                    self.unset("bucket_at_fountain");
                    self.print("You grab the bucket.");
                } else {
                    done = false;
                }
            }
            140 => {
                if m(n, &["card", "karte"]) && self.flag("roboss_short") && !self.has("card") && !self.has("card_coded") {
                    self.add("card");
                    self.award(25);
                    self.print("You take a blank code card from the delivery slot.");
                    self.set("oxygen_warning");
                    self.print("Somewhere an alarm begins to wail. The city's oxygen supply has been cut!");
                } else {
                    done = false;
                }
            }
            145 => {
                if m(n, &["shard", "splitter", "gem"]) && self.flag("museum_shard_ready") {
                    self.give_shard(2);
                    self.unset("museum_shard_ready");
                    self.award(50);
                    self.print("You take the second gem splinter from the output slot.");
                    self.enter(26);
                    self.print("The museum dissolves and you stand once more in the secret cellar.");
                } else {
                    done = false;
                }
            }
            146 => {
                if m(n, &["dung", "fladen"]) {
                    if self.has("leaves") && !self.has("dung") && !self.flag("dung_taken") {
                        self.add("dung");
                        self.set("dung_taken");
                        self.print("You wrap the dung in leaves and take it.");
                        self.award(10);
                    } else if self.has("dung") || self.flag("dung_taken") {
                        self.print("You already have the dung.");
                    } else {
                        self.print("It is too foul to touch. You need something to wrap it in.");
                    }
                } else {
                    done = false;
                }
            }
            147 => {
                if m(n, &["leaves", "leaf", "blaetter", "blätter"]) && !self.has("leaves") {
                    self.add("leaves");
                    self.print("You gather a handful of broad leaves from the footprints.");
                } else {
                    done = false;
                }
            }
            151 => {
                if m(n, &["meat", "fleisch"]) && self.flag("tiger_distracted") && !self.flag("meat_taken") {
                    self.add("meat");
                    self.set("meat_taken");
                    self.print("You take the haunch of meat.");
                    self.award(10);
                } else {
                    done = false;
                }
            }
            153 => {
                if m(n, &["shard", "splitter", "gem"]) && self.flag("spider_dropped") && !self.flag("shard5") {
                    self.give_shard(5);
                    self.award(50);
                    self.print("You take the fifth gem splinter. The baby dinosaur watches from the ferns.");
                    self.enter(26);
                    self.print("The prehistoric world dissolves, and you are back in the secret cellar.");
                } else {
                    done = false;
                }
            }
            157 => {
                if m(n, &["spider", "spinne"]) && !self.has("spider") && !self.flag("spider_taken") {
                    self.set("spider_attempts");
                    if self.flag("spider_second_try") {
                        self.add("spider");
                        self.set("spider_taken");
                        self.print("You take the monster spider. It is quite peaceful.");
                        self.award(20);
                    } else {
                        self.set("spider_second_try");
                        self.print("You reach for the spider. It shifts, and you draw back. Try again.");
                    }
                } else if m(n, &["bananas", "bananen"]) && !self.flag("bananas_taken") {
                    self.add("bananas");
                    self.set("bananas_taken");
                    self.print("You take the bunch of bananas.");
                    self.award(10);
                } else {
                    done = false;
                }
            }
            160 => {
                if m(n, &["pickaxe", "pickel", "hacke"]) && !self.flag("pickaxe_given") && !self.flag("mine_pickaxe_taken") {
                    if !self.has("pickaxe") {
                        self.add("pickaxe");
                    }
                    self.set("mine_pickaxe_taken");
                    self.print("You roll the exhausted man over and take his pickaxe.");
                    self.award(10);
                } else {
                    done = false;
                }
            }
            165 => {
                if m(n, &["shard", "splitter", "gem", "stones", "stein"]) && self.flag("sixth_shard_ready") && !self.flag("sixth_taken") {
                    self.give_shard(6);
                    self.set("sixth_taken");
                    self.award(50);
                    self.print("You search the ore stones and find the sixth gem splinter.");
                } else {
                    done = false;
                }
            }
            170 => {
                if m(n, &["eye", "auge"]) && self.flag("eye_ready") {
                    self.add("eye");
                    self.unset("eye_ready");
                    self.print("The smith holds out the Eye of Kal. You take it.");
                } else {
                    done = false;
                }
            }
            174 => {
                if m(n, &["amulet"]) && self.flag("amulet_offered") && !self.has("amulet") {
                    self.add("amulet");
                    self.award(10);
                    self.print("You take the bronze amulet. Something small mutters inside it.");
                } else {
                    done = false;
                }
            }
            176 => {
                if m(n, &["ball", "kugel"]) && self.flag("bridge_collapsed") && !self.has("ball") {
                    self.add("ball");
                    self.print("You lift the steel ball from the broken edge of the bridge.");
                } else if m(n, &["ball", "kugel"]) {
                    self.print("The beast watches. Taking the ball now would leave the bridge unstable.");
                } else {
                    done = false;
                }
            }
            175 => {
                if m(n, &["ball", "kugel"]) {
                    self.print("The beast watches. Taking the ball now would leave the bridge unstable.");
                } else {
                    done = false;
                }
            }
            178 => {
                if m(n, &["microphone", "mikrofon"]) && !self.has("microphone") {
                    self.add("microphone");
                    self.print("You take the microphone from the steps.");
                } else if m(n, &["speaker", "lautsprecher"]) && !self.has("speaker") {
                    self.add("speaker");
                    self.print("You prise the loudspeaker out of the amplifier box.");
                } else {
                    done = false;
                }
            }
            190 => {
                if m(n, &["wood", "holz"]) && self.flag("sand_dug") && !self.has("wood") {
                    self.add("wood");
                    self.print("You dig a waterlogged piece of wood out of the sand.");
                } else if m(n, &["stonewort", "plant", "root", "wurzel"]) && !self.has("root") && !self.has("vine_rope") {
                    let tries = if self.flag("stonewort_try2") { 2 } else if self.flag("stonewort_try1") { 1 } else { 0 };
                    if tries >= 1 {
                        self.set("stonewort_ready");
                        self.add("root");
                        self.print("You get a grip on the stonewort at last and pull it free. A long root vine comes with it.");
                    } else {
                        self.set("stonewort_try1");
                        self.print("The stonewort slips from your fingers. You will have to try again.");
                    }
                } else {
                    done = false;
                }
            }
            192 => {
                if m(n, &["oxygen", "gear", "geraet", "gerät"]) && self.flag("oxygen_revealed") && !self.has("oxygen") {
                    self.add("oxygen");
                    self.award(10);
                    self.print("You take the charged oxygen gear.");
                } else if m(n, &["wrench", "schluessel", "schlüssel", "english"]) && self.flag("oxygen_revealed") && !self.has("wrench") {
                    self.add("wrench");
                    self.print("You take the adjustable wrench from the cladding.");
                } else {
                    done = false;
                }
            }
            194 => {
                if m(n, &["scale", "schuppe"]) && self.flag("scale_fallen") && !self.has("scale") {
                    self.add("scale");
                    self.print("You take the reflective scale.");
                } else {
                    done = false;
                }
            }
            198 => {
                if m(n, &["soot", "russ"]) && self.flag("chair_moved") && !self.flag("soot_smeared") {
                    self.set("soot_smeared");
                    self.print("You climb the chair and take soot from the wall. You smear it over your face, hands and clothes until you are as black as the city.");
                } else {
                    done = false;
                }
            }
            234 => {
                if m(n, &["sigma", "cell", "zelle"]) && !self.has("sigma") {
                    self.add("sigma");
                    self.print("You take a sigma cell from the apparatus.");
                } else {
                    done = false;
                }
            }
            235 => {
                if m(n, &["key", "schluessel", "schlüssel"]) && !self.has("kal_key") && !self.flag("kal_switch_ready") {
                    self.add("kal_key");
                    self.print("You take the key from the north passage.");
                } else {
                    done = false;
                }
            }
            236 => {
                if m(n, &["eye", "auge"]) {
                    self.print("The Eye of Kal is not here. It is yours already.");
                } else {
                    done = false;
                }
            }
            _ => done = false,
        }

        if !done {
            if self.has(n.trim()) {
                self.print("You already have that.");
            } else {
                self.print("You don't see that here.");
            }
        }
    }

    fn put_down(&mut self, n: &str) {
        // Placement commands such as PUT FLY ON WEB, PUT SUGAR IN JAR.
        if n.contains(" on ") || n.contains(" in ") || n.contains(" into ") || n.contains(" at ") {
            self.place(n);
            return;
        }
        if m(n, &["jar"]) && self.has("jar") {
            if self.room == 9 && self.flag("sugar_in_jar") && !self.flag("fly_dead") {
                self.set("jar_on_floor");
                self.print("You set the open jar with the sugar cube on the floor of the hall and step back.");
            } else if self.room == 9 {
                self.set("jar_on_floor");
                self.print("You set the jar on the floor of the hall.");
            } else {
                self.print("You put the jar down.");
            }
            return;
        }
        if m(n, &["quicksand", "sand"]) && self.has("quicksand") && self.room == 139 {
            self.remove("quicksand");
            self.set("quicksand_placed");
            self.print("You scatter the quicksand near the fountain. A cleaning robot whirs over and sets down its bucket to deal with the mess.");
            self.set("bucket_free");
            return;
        }
        if m(n, &["bucket", "eimer"]) && self.has("bucket") && self.room == 139 {
            self.remove("bucket");
            self.set("bucket_at_fountain");
            self.print("You set the bucket at the foot of the fountain.");
            return;
        }
        if m(n, &["dung"]) && self.has("dung") && self.room == 155 {
            self.remove("dung");
            self.set("dung_dropped");
            self.print("You drop the dung on the river bank. It makes a useful landmark.");
            return;
        }
        if m(n, &["bananas"]) && self.has("bananas") && self.room == 151 {
            self.remove("bananas");
            self.set("bananas_dropped");
            self.print("You lay the bananas on the steppe.");
            return;
        }
        if m(n, &["meat"]) && self.has("meat") && self.room == 150 {
            self.remove("meat");
            self.set("meat_dropped");
            self.print("You lay the meat on the meadow.");
            return;
        }
        if m(n, &["spider"]) && self.has("spider") && self.room == 153 {
            self.remove("spider");
            self.set("spider_dropped");
            self.award(20);
            self.print("You set the monster spider down in front of the baby dinosaur.");
            self.print("The baby shrieks, drops the gem splinter and flees. You can take it now.");
            return;
        }
        if m(n, &["speaker"]) && self.has("speaker") && (self.room == 175 || self.room == 176) {
            self.remove("speaker");
            self.set("speaker_on_ball");
            self.print("You lay the loudspeaker on the steel ball.");
            return;
        }
        if m(n, &["ball", "kugel"]) && self.has("ball") && self.room == 179 {
            self.remove("ball");
            self.set("door_unlocked");
            self.award(10);
            self.print("You set the steel ball into the keyhole. The great door unlocks.");
            return;
        }
        if m(n, &["card", "karte"]) && (self.has("card") || self.has("card_coded")) {
            self.print("You put the card away.");
            return;
        }
        // Generic drop.
        let mut found = None;
        for id in self.inv.clone() {
            if n.contains(&item_name(id).to_lowercase()) || n.contains(id) {
                found = Some(id);
                break;
            }
        }
        if let Some(id) = found {
            self.remove(id);
            self.print(&format!("You put down the {}.", item_name(id)));
        } else {
            self.print("You aren't carrying that.");
        }
    }

    fn place(&mut self, n: &str) {
        if n.contains("speaker") && (n.contains("ball") || n.contains("kugel")) && (self.room == 175 || self.room == 176) && self.has("speaker") {
            self.remove("speaker");
            self.set("speaker_on_ball");
            self.print("You lay the loudspeaker on the steel ball, magnet downward.");
        } else if n.contains("ball") && (n.contains("keyhole") || n.contains("lock")) && self.room == 179 && self.has("ball") {
            self.remove("ball");
            self.set("door_unlocked");
            self.award(10);
            self.print("You set the steel ball into the keyhole. The great door unlocks.");
        } else if n.contains("powder") && (n.contains("cannon") || n.contains("gun")) && self.room == 66 {
            if self.has("powder") && self.has("rammer") {
                self.remove("powder");
                self.set("cannon_loaded");
                self.print("You ram the powder and ball home. The cannon is loaded.");
            } else {
                self.print("You need powder and a rammer.");
            }
        } else if n.contains("sugar") && n.contains("jar") {
            if self.has("sugar") && self.has("jar") {
                self.remove("sugar");
                self.set("sugar_in_jar");
                self.print("You drop the sugar cube into the jam jar and leave the lid open.");
            } else {
                self.print("You need both the sugar and the jar.");
            }
        } else if n.contains("fly") && (n.contains("web") || n.contains("net")) {
            if self.room != 8 {
                self.print("There is no web here.");
            } else if !self.has("fly") {
                self.print("You have no fly.");
            } else {
                self.remove("fly");
                self.set("fly_given");
                self.unset("spider_on_sponge");
                self.print("You lay the dead fly on the web. The spider pounces on it, and the way to the sponge is clear.");
                self.award(10);
            }
        } else if n.contains("wax") && (n.contains("sponge") || n.contains("cloth")) {
            if !self.has("wax") {
                self.print("You have no wax.");
            } else if !self.flag("wax_open") {
                self.print("Open the tube of wax first.");
            } else if n.contains("sponge") && !self.has("sponge") {
                self.print("You have no sponge.");
            } else {
                self.set("wax_on_sponge");
                self.print("You smear floor wax over the cloth.");
            }
        } else if n.contains("snow") && (n.contains("socket") || n.contains("eye")) {
            if self.room != 32 {
                self.print("There is no eye socket here.");
            } else {
                let count = if self.flag("snow_throw_3") { 3 } else if self.flag("snow_throw_2") { 2 } else if self.flag("snow_throw_1") { 1 } else { 0 };
                if count >= 3 {
                    self.print("The socket is full and the monolith is boiling inside.");
                } else {
                    self.set(&format!("snow_throw_{}", count + 1));
                    if count + 1 >= 3 {
                        self.set("monolith_hot");
                        self.print("The third handful of snow hisses into steam. The monolith's interior is boiling.");
                    } else {
                        self.print("The snow hisses and vanishes. The stone grows hotter.");
                    }
                }
            }
        } else if n.contains("shard") && (n.contains("socket") || n.contains("eye")) {
            if self.room != 32 || !self.flag("monolith_hot") {
                self.print("That is not possible here.");
            } else if !self.has_shard(1) {
                self.print("You have no splinter to place.");
            } else {
                self.set("shard_in_socket");
                self.print("You press your gem splinter into the eye socket.");
            }
        } else if n.contains("pole") && (n.contains("hole") || n.contains("plank")) {
            if self.has("pole") && self.room == 69 {
                self.remove("pole");
                self.set("pole_in_plank");
                self.print("You jam the pole into the knot hole. It stands upright like a mast.");
            } else {
                self.print("There is no hole for it here.");
            }
        } else if n.contains("coin") && (n.contains("plaque") || n.contains("panel") || n.contains("slot")) {
            if self.room == 1 && self.has("coin") {
                self.remove("coin");
                self.set("plaque_open");
                self.add("key");
                self.award(25);
                self.print("The coin drops into the plaque. A hidden drawer slides open, and inside is a small key.");
            } else {
                self.print("That does not fit here.");
            }
        } else if n.contains("shard") && n.contains("kal") {
            if self.room == 236 {
                self.print("Kal shakes his head. 'The splinters are already set in the Eye. Keep it - I want the machine freed, not the stone.'");
            } else {
                self.print("There is no one here to take it.");
            }
        } else if n.contains("bucket") && (n.contains("bottom") || n.contains("fountain") || n.contains("basin")) {
            if self.room == 139 && self.has("bucket") {
                self.remove("bucket");
                self.set("bucket_at_fountain");
                self.print("You set the bucket at the bottom of the fountain.");
            } else {
                self.print("You can't place it there.");
            }
        } else if n.contains("scale") && (n.contains("light") || n.contains("beam")) {
            if self.room == 188 && self.has("scale") {
                self.set("plate_open");
                self.print("You hold the reflective scale in the beam. The light strikes the rock plate, and it grinds open.");
            } else {
                self.print("There is no light beam here.");
            }
        } else {
            self.print("That doesn't work.");
        }
    }

    fn open(&mut self, n: &str) {
        if m(n, &["door"]) && self.room == 1 {
            if self.flag("postman") {
                self.print("The door is already open.");
            } else {
                self.set("postman");
                self.print("You open the front door. A postman thrusts a heavy envelope into your hands, asks two pointless questions, and hurries off.");
                self.print("TAKE ENVELOPE.");
            }
        } else if m(n, &["door"]) && self.room == 75 {
            self.set("blockhouse_open");
            self.print("You lift the latch and push the blockhouse door open.");
        } else if m(n, &["door"]) && self.room == 76 {
            self.print("The blockhouse door is already open.");
        } else if m(n, &["envelope", "umschlag"]) {
            if !self.has("envelope") {
                self.print("You don't have the envelope.");
            } else if !self.flag("cord_cut") {
                self.print("The envelope is tied with stout cord. CUT CORD WITH BLADE.");
            } else if self.flag("envelope_open") {
                self.print("The envelope is already open.");
            } else {
                self.set("envelope_open");
                self.add("cord");
                self.add("letter");
                self.add("diary");
                self.give_shard(1);
                self.print("You open the envelope. Inside are a letter, a diary, and a gem splinter that seems to hold a cold light.");
                self.print("TAKE SHARD.");
            }
        } else if m(n, &["box", "control box"]) && self.room == 139 {
            self.set("fountain_box_open");
            self.print("You open the fountain's control box. A single switch waits inside.");
        } else if m(n, &["tin", "dose", "can"]) && self.has("tin") {
            self.set("tin_opened");
            self.print("You pry the lid off the breakfast tin. A little key lies inside.");
        } else if m(n, &["crate", "box"]) && self.room == 27 {
            self.set("crate_open");
            self.print("You lever the crate open. Inside are fur boots and a heavy fur coat.");
        } else if m(n, &["bible", "buch"]) && self.has("bible") {
            if self.flag("bible_open") {
                self.print("The bible's spine is already split open.");
            } else {
                self.set("bible_open");
                self.print("You work the spine of the children's bible apart. A silver coin is sewn into the binding. TAKE COIN.");
            }
        } else if m(n, &["jar"]) && self.has("jar") {
            if self.flag("jar_closed") {
                self.unset("jar_closed");
                self.print("You unscrew the lid of the jar.");
            } else {
                self.print("The jar is already open.");
            }
        } else if m(n, &["wax", "tube"]) && self.has("wax") {
            self.set("wax_open");
            self.print("You squeeze wax out of the tube.");
        } else if m(n, &["piano", "lid", "deckel"]) && self.room == 3 {
            if self.flag("piano_lid") {
                self.print("The lid is already open.");
            } else {
                self.set("piano_lid");
                self.print("You lift the piano lid. The keyboard flap is still locked.");
            }
        } else if m(n, &["flap", "lock", "keyhole"]) && self.room == 3 {
            if !self.flag("piano_lid") {
                self.print("Open the piano lid first.");
            } else if self.has("key") && !self.flag("piano_unlocked") {
                self.set("piano_unlocked");
                self.print("The little key turns. The keyboard flap springs open.");
            } else if self.flag("piano_unlocked") {
                self.print("The keyboard flap is open.");
            } else {
                self.print("It is locked, and the keyhole is tiny.");
            }
        } else if m(n, &["buffet", "drawer"]) && self.room == 13 {
            self.set("buffet_open");
            self.print("The drawer comes open. Inside is a sugar cube.");
        } else if m(n, &["compartment", "tool"]) && self.room == 121 {
            self.set("compartment_open");
            self.print("The compartment opens. A hammer and a crowbar are inside.");
        } else if m(n, &["cover", "plate"]) && self.room == 122 {
            if self.has("passkey") {
                self.set("cover_open");
                self.print("The passkey turns. The cover plate comes away, exposing the maintenance unit.");
            } else {
                self.print("The cover is held by a recessed screw. A passkey would fit.");
            }
        } else if m(n, &["gate", "grille"]) && self.room == 35 {
            if self.flag("icegate_open") {
                self.print("The gate already stands open.");
            } else if self.has("icekey") {
                self.set("icegate_open");
                self.print("The iron key turns in the great lock. The gate swings open.");
            } else {
                self.print("The gate is locked. A large iron key might fit.");
            }
        } else if m(n, &["chest", "sea chest"]) && self.room == 74 {
            if self.flag("chest_lid_open") {
                self.print("The chest is already open.");
            } else if self.flag("chest_pulled") {
                self.set("chest_lid_open");
                self.print("You force the lid open. Under a layer of oilcloth is a packet of black powder.");
                self.print("TAKE POWDER.");
            } else {
                self.print("The chest is still out in the water.");
            }
        } else if m(n, &["amulet"]) && self.has("amulet") {
            if self.room != 181 {
                self.print("The smith warned you: the amulet must only be opened in the black crypt.");
            } else if self.flag("pit_out") {
                self.print("The amulet is already open.");
            } else {
                self.set("pit_out");
                self.award(25);
                self.print("You open the amulet. A tiny kobold tumbles out, sneezes, and climbs onto your shoulder.");
                self.print("\"Pit, at your service. I smell a riddle.\"");
            }
        } else if m(n, &["door", "black door"]) && self.room == 199 {
            if self.has("boathook") {
                self.set("blackway_door");
                self.print("You wedge the boat hook into the seam and heave. The black door groans open.");
            } else {
                self.print("The door has no handle. A long lever might force it.");
            }
        } else if m(n, &["door", "gate"]) && self.room == 203 {
            self.print("The gate is controlled by the button.");
        } else {
            self.print("You can't open that.");
        }
    }

    fn close(&mut self, n: &str) {
        if m(n, &["jar"]) && self.has("jar") {
            if self.room == 9 && self.flag("jar_on_floor") && self.flag("fly_in_jar") && !self.flag("fly_dead") {
                self.set("fly_dead");
                self.set("jar_closed");
                self.award(10);
                self.print("You snatch up the jar and screw the lid on. The fly buzzes once, then goes still. TAKE FLY.");
            } else {
                self.set("jar_closed");
                self.print("You screw the lid onto the jar.");
            }
        } else if m(n, &["gate", "steel gate"]) && self.room == 195 {
            self.set("gate_closed");
            self.unset("gate_open");
            if self.flag("kathaura_lured") {
                self.set("kathaura_trapped");
                self.print("You slam the steel gate shut. Kathaura hammers on it from the sluice - and then the monster's shadow rises behind him.");
            } else {
                self.print("The steel gate closes with a clang.");
            }
        } else if m(n, &["grille", "gate"]) && self.room == 195 {
            self.set("grille_closed");
            self.unset("grille_open");
            self.print("The grille shuts.");
        } else if m(n, &["mouth", "bool"]) && self.room == 192 {
            self.set("mouth_closed");
            self.unset("bool_open");
            self.print("You press the mouth button. The Bool's jaws close.");
        } else if m(n, &["door"]) && self.room == 1 {
            self.print("It is already closed - or open. Best leave it.");
        } else {
            self.print("You can't close that.");
        }
    }

    fn read(&mut self, n: &str) {
        if m(n, &["diary", "journal", "tagebuch"]) && self.has("diary") {
            self.set("diary_read");
            self.show_journal();
        } else if m(n, &["letter", "brief"]) && self.has("letter") {
            self.print("'Harold - the Eye of Kal was never a myth. Six splinters, six worlds. The stone circle is the door. Trust the diary. - A.'");
        } else if m(n, &["note", "paper", "zettel"]) && self.has("note") {
            self.print("A note in a cramped hand: 'The portrait hides a tune. Five notes open the piano. The last is sewn into plush.'");
            self.set("note_read");
        } else if m(n, &["paper", "warning"]) && self.room == 76 {
            self.print("'ADMIRAL ATTERSON - do not enter the jungle. The blossoms are not what they seem. - A friend.'");
        } else if m(n, &["paper"]) && self.room == 181 {
            self.print("'Every chamber of the labyrinth has six ways out. Only the riddles of Pit lead through. Eleven chambers stand between the crypt and the lake.'");
            self.set("labyrinth_known");
        } else if m(n, &["sign"]) && self.room == 35 {
            self.print("'ICE CAVE - KEEP OUT. BEARS.'");
        } else if m(n, &["plaque"]) && self.room == 1 {
            self.print("'H. JUGGER - THE EYE SEES ALL.' A coin slot waits beneath the words.");
        } else if m(n, &["nameplate"]) && self.room == 236 {
            self.print("The nameplate reads: KAL - KEEPER OF THE EYE.");
        } else if m(n, &["spell", "wall"]) && self.room == 200 {
            self.print("'To walk unseen in the black city, take the colour of the stone into your skin.'");
            self.set("spell_read");
        } else if m(n, &["plaque"]) && self.room == 195 {
            self.print("A brass plaque: 'NORTH SLUICE - KATHURA'S WATCH. AUTHORISED PERSONNEL ONLY.'");
            self.set("sluice_solved");
        } else if m(n, &["note", "sheet", "music"]) && self.has("note") {
            self.print("The paper bears a fragment of music: GIS, GES, DIS, FIS ... and a blank where a fifth note should be.");
            self.set("music_hint");
        } else {
            self.print("There is nothing written there.");
        }
    }

    fn polish(&mut self, n: &str) {
        if self.room != 2 && !n.contains("floor") {
            self.print("There is no floor here that needs polishing.");
            return;
        }
        if self.room == 2 && self.has("toothbrush") && self.flag("floor_waxed") {
            self.set("floor_polished");
            self.print("You scrub the waxed boards with the toothbrush until they gleam like glass.");
            self.print("The couch would slide now - PUSH COUCH.");
        } else if self.room == 2 && self.has("sponge") && self.flag("wax_on_sponge") {
            self.set("floor_waxed");
            self.print("You rub the waxed sponge over the library floor. The boards grow slick, but dull.");
        } else if self.room == 2 {
            self.print("The floor needs wax before it can be polished.");
        } else {
            self.print("You polish. Nothing comes of it.");
        }
    }

    fn board(&mut self, n: &str) {
        let room = self.room;
        if room == 163 {
            self.print("You are already in the mine cart.");
        } else if room == 162 || room == 165 {
            self.enter(163);
            self.print("You climb into the mine cart.");
        } else if room == 67 {
            self.climb("plank");
        } else if room == 69 {
            self.print("You are already aboard the plank.");
        } else {
            self.print(&format!("There is nothing called '{}' to board here.", n));
        }
    }

    fn drink(&mut self, n: &str) {
        if m(n, &["elixir", "flask"]) && self.has("elixir") {
            self.print("This elixir is not for you. Keep it for the exhausted slave in the mine.");
        } else if m(n, &["water"]) {
            self.print("You cup your hands and drink. It helps a little.");
        } else {
            self.print("You are not thirsty.");
        }
    }

    fn press(&mut self, n: &str) {
        if self.room == 135 {
            self.srt_panel(n);
            return;
        }
        if self.room == 26 {
            self.stone_circle(n);
        } else if self.room == 32 && m(n, &["shard", "splitter", "socket", "eye"]) {
            self.turn(n);
        } else if m(n, &["table", "tisch"]) && self.room == 10 {
            self.set("table_in_hall");
            self.print("You shove the kitchen table east through the door. It slides across the upstairs hall floor.");
        } else if m(n, &["table", "tisch"]) && self.room == 9 {
            if self.flag("table_in_hall") || self.flag("table_under_trapdoor") {
                self.set("table_under_trapdoor");
                self.print("You push the table until it stands directly under the trapdoor.");
            } else {
                self.print("There is no table in the hall.");
            }
        } else if m(n, &["couch", "sofa"]) && self.room == 2 {
            if self.flag("couch_moved") {
                self.print("The couch has already been moved aside.");
            } else if self.flag("floor_polished") {
                self.set("couch_moved");
                self.print("You heave the couch aside. A folded sheet of paper lies on the polished floor. TAKE PAPER.");
            } else {
                self.print("The couch will not slide on the dull floor. It needs wax and a thorough polish.");
            }
        } else if m(n, &["handle", "hebel", "griff"]) && self.room == 24 {
            if !self.flag("handle_inserted") {
                self.print("The slot beside the broken lever is empty.");
            } else if !self.flag("monster_shut") {
                self.set("monster_shut");
                self.print("You press the handle up. The stone face grinds shut above you, sealing the monster out.");
            } else {
                self.set("pit_open");
                self.add("handle");
                self.print("You press the handle down. With a rumble the wall swings aside, opening the way to the secret cellar.");
                self.print("You work the wooden handle free again - it will serve as a sword grip once more.");
            }
        } else if m(n, &["handle", "hebel", "griff"]) && self.room == 26 {
            self.print("The handle is a lever in the black pit, not here.");
        } else if m(n, &["brick", "stein"]) && self.room == 5 {
            self.set("brick_open");
            self.print("You press the loose brick. With a grinding rumble, the back of the fireplace slides aside and a stair leads down.");
        } else if m(n, &["button", "knopf"]) && self.room == 124 {
            self.intercom_boston();
        } else if m(n, &["button", "knopf"]) && self.room == 128 {
            self.intercom_baltimore();
        } else if m(n, &["button", "knopf"]) && self.room == 140 {
            if self.flag("roboss_short") {
                self.set("vending_ready");
                self.print("You press the button. Something clunks in the delivery slot.");
            } else {
                self.print("The Roboss blocks the vending machine.");
            }
        } else if m(n, &["button", "knopf"]) && self.room == 126 {
            if self.flag("plug_attached") {
                self.set("power_on");
                self.print("Power hums through the repair vehicle.");
            } else {
                self.print("Nothing happens. The machine has no power.");
            }
        } else if m(n, &["dead man", "switch", "taste"]) && self.room == 126 {
            if !self.flag("power_on") {
                self.print("The controls are dead.");
            } else {
                self.set("deadman_held");
                self.print("You hold the dead-man switch down. The robot arm twitches.");
            }
        } else if m(n, &["button", "knopf", "switch", "schalter"]) && self.room == 139 {
            if self.flag("bucket_at_fountain") {
                self.set("fountain_on");
                self.print("You press the switch. Water bursts from the copper nozzles.");
            } else {
                self.print("The control box is open, but nothing happens.");
            }
        } else if m(n, &["button", "knopf"]) && self.room == 145 {
            self.set("museum_prompt");
            self.print("The terminal asks for the three-digit output code. SAY <number>.");
        } else if m(n, &["button", "knopf"]) && self.room == 195 {
            self.set("grille_open");
            self.unset("grille_closed");
            self.print("The grille opens.");
        } else if m(n, &["key", "taste"]) && self.room == 195 {
            self.set("gate_open");
            self.unset("gate_closed");
            self.print("The steel gate opens.");
        } else if m(n, &["mouth", "maul"]) && self.room == 192 {
            self.set("bool_open");
            self.unset("mouth_closed");
            self.print("You press the mouth button. The Bool's jaws grind open.");
        } else if m(n, &["chair", "stuhl", "thron"]) && (self.room == 200 || self.room == 198) {
            if self.flag("chair_moved") {
                self.print("The chair already stands beneath the sooty wall.");
            } else {
                self.set("chair_moved");
                self.print("You shove the giant chair west, then south into Kathaura's watch, until it stands beneath the blackened wall.");
            }
        } else if m(n, &["button", "knopf"]) && self.room == 202 {
            if self.flag("soot_smeared") {
                self.set("khalin_called");
                self.print("You press the button and wait. Khalin's enormous face appears at the window, and the gate below begins to open.");
            } else {
                self.print("A voice thunders: 'WHO IS THERE?' You had better not be seen.");
            }
        } else if m(n, &["button", "knopf"]) && self.room == 236 {
            self.kal_button();
        } else if m(n, &["button", "knopf"]) && self.room == 134 {
            self.print("You press the call button. The SRT door sighs open.");
            self.set("srt_door_open");
        } else {
            self.print("Pressing that has no effect.");
        }
    }

    fn pull(&mut self, n: &str) {
        if m(n, &["plate", "floor", "bodenplatte"]) && self.room == 16 {
            self.set("cellar_plate_open");
            self.print("You haul on the stone plate. It comes up, revealing a black shaft.");
        } else if m(n, &["lever", "hebel"]) && self.room == 126 {
            if self.flag("deadman_held") && self.flag("power_on") {
                self.set("tongs_fallen");
                self.print("You hold the dead-man switch and push the lever. The robot arm sweeps around; the tongs clatter to the platform below.");
            } else {
                self.print("The lever moves, but the arm has no power.");
            }
        } else if m(n, &["lever", "hebel"]) && self.room == 24 {
            if !self.flag("lever_broken") {
                self.set("lever_broken");
                self.print("You pull the old stone lever. It snaps off in your hand.");
            } else if self.has("handle") && !self.flag("handle_inserted") {
                self.remove("handle");
                self.set("handle_inserted");
                self.print("You jam the wooden handle into the slot where the lever broke.");
            } else {
                self.print("The replacement handle is already in place.");
            }
        } else if m(n, &["handle", "hebel"]) && self.room == 24 {
            if self.has("handle") && !self.flag("handle_inserted") {
                self.remove("handle");
                self.set("handle_inserted");
                self.print("You fit the wooden handle into the slot.");
            } else {
                self.print("There is nothing to pull.");
            }
        } else if m(n, &["plate", "floor"]) && self.room == 233 {
            self.set("pavilion_plate_open");
            self.print("The stone slab slides aside, opening a hole in the pavilion floor.");
        } else if m(n, &["chest", "sea chest"]) && self.room == 74 {
            if self.has("boathook") {
                self.set("chest_pulled");
                self.print("You hook the sea chest with the boat hook and drag it ashore.");
            } else {
                self.print("The chest is out of reach. A long hook would help.");
            }
        } else if m(n, &["rope", "seil"]) && (self.room == 175 || self.room == 176 || self.room == 177) {
            if self.flag("speaker_on_ball") && self.has("rope") {
                self.remove("rope");
                self.set("bridge_collapsed");
                self.print("You pull the rope. The loudspeaker's magnet jerks the ball free, and the bridge collapses into the chasm.");
                self.print("The steel ball remains on the near edge. TAKE BALL.");
            } else {
                self.print("You would need something to pull with.");
            }
        } else if m(n, &["drawbar", "lever", "zugriegel"]) && self.room == 198 {
            self.pull_drawbar();
        } else if m(n, &["lever"]) && self.room == 160 {
            self.print("The cart lever is fixed to the chassis.");
        } else {
            self.print("You pull, but nothing gives.");
        }
    }

    fn turn(&mut self, n: &str) {
        if self.room == 3 && m(n, &["key"]) {
            self.open("key flap");
        } else if self.room == 121 && m(n, &["key", "ignition", "zuendschloss"]) {
            if self.has("car_key") {
                self.set("engine_on");
                self.print("You insert the ignition key and turn it. The hovercraft's engine whines to life.");
            } else {
                self.print("The ignition is empty.");
            }
        } else if self.room == 121 && m(n, &["knob", "knopf", "dial"]) {
            if !self.flag("engine_on") {
                self.print("The engine is dead.");
            } else if m(n, &["left", "links"]) {
                if self.flag("hover_fixed") {
                    self.award(25);
                    self.print("You turn the knob left. The hovercraft lifts and glides away to a parking lot in Baltimore.");
                    self.enter(128);
                } else {
                    self.set("knob_left");
                    self.print("You turn the knob left. A voice asks whether you want to drive to Boston. The knob sticks.");
                }
            } else if m(n, &["right", "rechts"]) {
                if self.flag("knob_left") && self.flag("hover_damaged") && !self.flag("boston_done") {
                    self.award(25);
                    self.set("boston_done");
                    self.print("You press the switch down and turn the knob right. The hovercraft surges away and sets down at the Boston repair depot.");
                    self.enter(124);
                } else {
                    self.print("The knob will not turn that way yet.");
                }
            } else {
                self.print("Left or right?");
            }
        } else if self.room == 32 && m(n, &["shard", "splitter", "socket", "eye"]) {
            if self.flag("shard_in_socket") {
                self.award(25);
                self.set("monolith_gone");
                self.lose_shard(1);
                self.print("You twist the splinter in the socket. Something inside the monolith cracks open, and you barely get clear before the whole stone erupts.");
                self.print("You stagger east and south. When the dust settles, a deep shaft gapes where the monolith stood.");
                self.enter(33);
            } else {
                self.print("There is no splinter in the socket.");
            }
        } else if self.room == 195 && m(n, &["regulator", "regler", "wheel", "valve"]) {
            if m(n, &["on", "open", "one", "1", "auf", "fill"]) {
                self.set("sluice_filled");
                self.unset("sluice_empty");
                self.print("The regulator turns. Water begins to flood the sluice.");
            } else {
                self.set("sluice_empty");
                self.unset("sluice_filled");
                self.print("The regulator turns the other way. The sluice begins to drain.");
            }
        } else if self.room == 192 && m(n, &["switch", "schalter"]) {
            if m(n, &["left", "links"]) {
                self.set("cladding_open");
                self.set("oxygen_revealed");
                self.print("You turn the switch left. The metal cladding unlocks and falls open, revealing an oxygen gear.");
            } else if m(n, &["right", "rechts"]) {
                if self.flag("cladding_open") {
                    self.set("oxygen_charged");
                    self.print("You turn the switch right. The oxygen gear hisses as it charges.");
                } else {
                    self.print("The cladding is still locked.");
                }
            } else {
                self.print("Left or right?");
            }
        } else if self.room == 194 && m(n, &["bolt", "bolzen"]) {
            if self.has("wrench") {
                self.set("scale_fallen");
                self.print("You fit the wrench to the bolt and turn. It comes free, and a great scale peels away and falls toward the lake bed.");
            } else {
                self.print("The bolt is too tight for your fingers. You need a wrench.");
            }
        } else if self.room == 87 && m(n, &["crank", "kurbel", "winch"]) {
            self.set("winch_turned");
            self.print("You turn the crank. The winch ropes tighten, but the bridge stays raised.");
        } else if self.room == 8 && m(n, &["sponge", "washcloth"]) {
            if self.flag("wax_on_sponge") {
                self.set("sponge_turned");
                self.unset("spider_on_sponge");
                self.print("You turn the waxed sponge over. The spider recoils from the wax and scuttles away.");
            } else {
                self.print("You turn the sponge. The spider is not impressed.");
            }
        } else if self.room == 126 && m(n, &["lever", "hebel"]) {
            if self.flag("deadman_held") && self.flag("power_on") {
                self.set("tongs_fallen");
                self.print("You hold the dead-man switch and push the lever. The robot arm sweeps around; the tongs clatter to the platform below.");
            } else {
                self.print("The lever moves, but the arm has no power.");
            }
        } else if self.room == 24 && m(n, &["handle"]) {
            if self.flag("handle_inserted") {
                self.set("pit_open");
                self.print("You press the handle down. The stone wall swings aside, opening the way to the secret cellar.");
            } else {
                self.print("The slot is empty.");
            }
        } else {
            self.print("You turn it, but nothing happens.");
        }
    }

    fn play(&mut self, n: &str) {
        if self.room == 4 && m(n, &["billiard", "pool", "table", "game"]) {
            if self.flag("table_damaged") {
                self.print("The table is already scarred.");
            } else if !self.flag("billiard_asked") {
                self.set("billiard_asked");
                self.print("You chalk a cue and line up a shot. Are you sure you want to risk damaging the table? YES or NO.");
            } else {
                self.print("The table waits. YES or NO?");
            }
        } else if self.room == 3 && m(n, &["piano", "gis", "ges", "dis", "fis", "ces", "music"]) {
            if !self.flag("piano_unlocked") {
                self.print("The keyboard flap is locked.");
            } else if self.has("note") && self.has("cloth") {
                self.set("piano_played");
                self.add("marble");
                self.award(25);
                self.print("You play GIS, GES, DIS, FIS, CES. The last note hangs in the air, and something rolls down inside the piano.");
                self.print("You reach in and take a marble ball.");
            } else {
                self.print("You do not know the tune yet. The portrait and the plush cloth hold the notes.");
            }
        } else {
            self.print("You can't play that.");
        }
    }

    fn use_on(&mut self, n: &str) {
        if m(n, &["terminal", "computer"]) && self.room == 137 {
            self.set("terminal_mode");
            self.print("The terminal wakes. It asks for a keyword. SAY <word>.");
        } else if m(n, &["terminal", "computer"]) && self.room == 145 {
            self.set("museum_prompt");
            self.print("The museum terminal asks for the three-digit output code. SAY <number>.");
        } else if m(n, &["lighter"]) && self.has("lighter") {
            self.light("lighter");
        } else if m(n, &["key"]) && self.room == 121 {
            self.turn("key");
        } else if n.contains(" on ") || n.contains(" with ") {
            // USE X ON Y behaves like ATTACH X TO Y.
            let t = n.replace(" on ", " to ").replace(" with ", " to ");
            self.attach_cmd(&t);
        } else if m(n, &["sword", "blade"]) {
            self.print("The blade is sharp. CUT <thing> is the way to use it.");
        } else if m(n, &["rope"]) {
            self.print("ATTACH ROPE TO <thing>.");
        } else {
            self.print("That does not seem to do anything.");
        }
    }

    fn attach_cmd(&mut self, n: &str) {
        let (main, third) = match n.split_once(" with ") {
            Some((a, b)) => (a.trim().to_string(), Some(b.trim().to_string())),
            None => (n.to_string(), None),
        };
        let (a, b) = match main.split_once(" to ") {
            Some((x, y)) => (x.trim().to_string(), y.trim().to_string()),
            None => {
                self.print("Attach what to what?");
                return;
            }
        };
        let has_a = self.inv.iter().any(|i| a.contains(*i) || a.contains(&item_name(i).to_lowercase()));
        let has_b = self.inv.iter().any(|i| b.contains(*i) || b.contains(&item_name(i).to_lowercase()));

        // Special rigs.
        if a.contains("cue") && b.contains("pole") && third.as_deref().map(|t| t.contains("cord") || t.contains("rope")).unwrap_or(false) {
            self.remove("cue");
            self.remove("rope");
            self.set("mast_rigged");
            self.award(10);
            self.print("You lash the billiard cue to the pole with the cord. The sail stretches tight.");
            return;
        }
        if (a.contains("sail") && b.contains("pole")) || (a.contains("pole") && b.contains("sail")) {
            if self.has("sail") && (self.has("pole") || self.flag("pole_in_plank")) {
                self.remove("sail");
                if self.has("pole") {
                    self.remove("pole");
                }
                self.set("sail_rigged");
                self.award(10);
                self.print("You tie the sailcloth to the pole. Your plank has a mast and a sail.");
            } else {
                self.print("You need both the sail and the pole.");
            }
            return;
        }
        if a.contains("rope") && b.contains("speaker") || a.contains("speaker") && b.contains("rope") {
            if self.has("rope") && self.has("speaker") {
                self.set("speaker_rope");
                self.print("You tie the rope to the loudspeaker, leaving a good length to pull with.");
            } else {
                self.print("You need both the rope and the loudspeaker.");
            }
            return;
        }
        if a.contains("speaker") && b.contains("rope") {
            self.print("You need both.");
            return;
        }
        if a.contains("root") && b.contains("wood") || a.contains("wood") && b.contains("root") {
            if self.has("root") && self.has("wood") {
                self.remove("root");
                self.remove("wood");
                self.add("vine_rope");
                self.print("You tie the root vine to the piece of wood. It makes a long diving line.");
            } else {
                self.print("You need both the vine and the wood.");
            }
            return;
        }
        if a.contains("tongs") && (b.contains("thread") || b.contains("gewinde")) {
            if self.has("tongs") {
                self.remove("tongs");
                self.remove("plug");
                self.set("hover_fixed");
                self.print("You take the plug and use the tongs to tear the damaged thread out of the socket. The connection is clean again.");
            } else {
                self.print("You need the tongs.");
            }
            return;
        }
        if a.contains("plug") && (b.contains("thread") || b.contains("gewinde") || b.contains("unit")) {
            if self.has("plug") && self.room == 122 {
                self.remove("plug");
                self.set("plug_attached");
                self.award(10);
                self.print("You screw the plug onto the thread. The maintenance unit has power.");
            } else {
                self.print("You can't attach it here.");
            }
            return;
        }
        if a.contains("wrecking") && (b.contains("debris") || b.contains("trümmer") || b.contains("trummer")) {
            if self.has("wrecking") && self.room == 33 {
                self.set("rope_anchor");
                self.print("You hook the wrecking ball onto the hot debris and pay the rope over the edge.");
            } else {
                self.print("There are no debris to anchor to here.");
            }
            return;
        }
        if (a.contains("rope") || a.contains("cord") || a.contains("grapple") || a.contains("wrecking")) && (b.contains("lever") || b.contains("hebel")) && self.room == 162 {
            if self.flag("cart_rigged") {
                self.print("The rope is already tied to the lever.");
            } else if self.has("rope") || self.has("cord") || self.has("grapple") || self.has("wrecking") {
                self.set("cart_rigged");
                self.award(10);
                self.print("You tie a line to the cart's lever. A sharp pull will tip the cart when it stops.");
            } else {
                self.print("You need a rope or a grappling line.");
            }
            return;
        }
        if (a.contains("cue") || a.contains("boathook") || a.contains("grapple") || a.contains("shaft")) && (b.contains("winch") || b.contains("winde") || b.contains("crank") || b.contains("kurbel")) {
            let lever = if self.has("boathook") { Some("boathook") } else if self.has("cue") { Some("cue") } else if self.has("grapple") { Some("grapple") } else if self.has("shaft") { Some("shaft") } else { None };
            if lever.is_some() {
                if self.room == 88 {
                    self.set("winch_jammed");
                    self.set("rockdoor_open");
                    self.print("You haul on the crank and jam the spar through the winch. The bridge rises and the rock door swings open. You can pull the spar free afterwards.");
                } else {
                    self.print("You can't do that here.");
                }
            } else {
                self.print("You need a stout spar to jam the winch.");
            }
            return;
        }
        if a.contains("rope") && (b.contains("pole") || b.contains("mast") || b.contains("stange")) {
            if (self.has("rope") || self.has("cord")) && (self.has("pole") || self.flag("pole_in_plank")) && self.room == 69 {
                if self.has("cord") {
                    self.remove("cord");
                } else {
                    self.remove("rope");
                }
                self.set("mast_rigged");
                self.award(10);
                self.print("You lash the spar to the pole with the cord. The sail stretches tight.");
            } else {
                self.print("You can't rig that here.");
            }
            return;
        }
        if a.contains("tongs") && (b.contains("winch") || b.contains("thread")) {
            self.print("That is not the right place.");
            return;
        }
        // Ordinary crafting.
        if !has_a || !has_b {
            self.print("You need both parts.");
            return;
        }
        self.craft(&a, &b);
    }

    fn craft(&mut self, a: &str, b: &str) {
        let pair = |p: &str, q: &str| {
            (a.contains(p) && b.contains(q)) || (a.contains(q) && b.contains(p))
        };
        if pair("blade", "handle") {
            self.remove("blade");
            self.remove("handle");
            self.add("sword");
            self.print("You set the blade into the wooden handle. A sword!");
        } else if pair("shaft", "handle") {
            self.remove("shaft");
            self.remove("handle");
            self.add("cue_shaft");
            self.print("You fit the shaft into the handle. It is starting to look like a billiard cue.");
        } else if pair("tip", "cue_shaft") || pair("tip", "shaft") {
            self.remove("tip");
            if self.has("cue_shaft") {
                self.remove("cue_shaft");
            } else {
                self.remove("shaft");
            }
            self.add("cue");
            self.print("You screw the brass tip onto the shaft. A complete billiard cue.");
        } else if pair("hook", "shaft") {
            self.remove("hook");
            self.remove("shaft");
            self.add("grapple");
            self.print("You lash the iron hook to the shaft. A grappling hook.");
        } else if pair("grapple", "handle") {
            self.remove("grapple");
            self.remove("handle");
            self.add("boathook");
            self.print("You add the handle to the grappling hook. A long boat hook.");
        } else if pair("rope", "grapple") {
            self.remove("rope");
            self.remove("grapple");
            self.add("wrecking");
            self.print("You tie the rope to the grappling hook. A wrecking ball.");
        } else if pair("flag", "club") {
            if self.room == 27 {
                self.remove("flag");
                self.remove("club");
                self.add("torch");
                self.award(10);
                self.print("You tie the flag around the club and dip it in the whale oil. A torch.");
            } else {
                self.print("You tie the flag to the club, but it needs oil to burn. The barrel in the ice niche would do.");
            }
        } else if pair("flag", "torch") || pair("club", "torch") {
            self.print("The torch is already assembled.");
        } else {
            self.print("Those two do not fit together.");
        }
    }

    fn detach(&mut self, n: &str) {
        if m(n, &["cue"]) && self.has("cue") {
            self.remove("cue");
            self.add("tip");
            self.add("shaft");
            self.add("handle");
            self.print("You unscrew the cue into tip, shaft and handle.");
        } else if m(n, &["cue_shaft"]) && self.has("cue_shaft") {
            self.remove("cue_shaft");
            self.add("shaft");
            self.add("handle");
            self.print("You separate the shaft from the handle.");
        } else if m(n, &["grapple"]) && self.has("grapple") {
            self.remove("grapple");
            self.add("hook");
            self.add("shaft");
            self.print("You untie the hook from the shaft.");
        } else if m(n, &["boathook", "boat hook"]) && self.has("boathook") {
            self.remove("boathook");
            self.add("grapple");
            self.add("handle");
            self.print("You take the boat hook apart into a grappling hook and a handle.");
        } else if m(n, &["sword"]) && self.has("sword") {
            self.remove("sword");
            self.add("blade");
            self.add("handle");
            self.print("You pull the blade out of the handle.");
        } else if m(n, &["wrecking"]) && self.has("wrecking") {
            self.remove("wrecking");
            self.add("rope");
            self.add("grapple");
            self.print("You untie the rope from the grappling hook.");
        } else if m(n, &["torch"]) && self.has("torch") {
            self.remove("torch");
            self.add("flag");
            self.add("club");
            self.print("You untie the flag from the club.");
        } else {
            self.print("You can't dismantle that.");
        }
    }

    fn cut(&mut self, n: &str) {
        if m(n, &["rope", "seil", "chandelier", "lustre", "lüster"]) && self.room == 1 && !self.flag("rope_cut") {
            if self.has("sword") || self.has("blade") {
                self.set("rope_cut");
                self.award(10);
                self.print("You cut the chandelier rope. The great chandelier crashes down; a candle and a length of rope lie in the wreck.");
            } else {
                self.print("You need a sharp blade to cut the rope.");
            }
        } else if m(n, &["cord", "envelope", "string", "schnur"]) && self.has("envelope") && !self.flag("cord_cut") {
            if self.has("blade") || self.has("sword") {
                self.set("cord_cut");
                self.award(10);
                self.print("You sever the envelope cord with the blade. OPEN ENVELOPE.");
            } else {
                self.print("You need a sharp blade.");
            }
        } else if m(n, &["thicket", "dickicht", "bush"]) && self.room == 75 {
            if self.has("sword") || self.has("blade") {
                self.set("thicket_cut");
                self.print("You hack a path through the thicket.");
            } else {
                self.print("The thicket is too dense for bare hands.");
            }
        } else if m(n, &["coconut", "kokos"]) && self.room == 71 {
            if self.has("sword") {
                self.add("coconut");
                self.set("palm_scratched");
                self.print("You cut the coconut free with the sword and catch it. The climb has left your arms and legs badly scratched.");
            } else {
                self.print("You can't reach it with your bare hands.");
            }
        } else if m(n, &["dung", "fladen"]) && self.room == 146 {
            if self.has("sword") || self.has("blade") {
                self.set("dung_cut");
                self.print("You cut the dung heap apart. The pieces are still too foul to touch.");
            } else {
                self.print("You need something sharp.");
            }
        } else if m(n, &["cable", "wire", "kabel"]) && self.room == 195 {
            if self.has("sword") {
                self.set("cable195_cut");
                self.print("You sever the sluice control cable with the sword.");
            } else {
                self.print("You need a blade.");
            }
        } else if m(n, &["cable", "wire", "kabel"]) && self.room == 198 {
            if self.has("sword") {
                self.set("cable198_cut");
                self.print("You sever the control cable in Kathaura's watch.");
            } else {
                self.print("You need a blade.");
            }
        } else {
            self.print("You can't cut that.");
        }
    }

    fn throw(&mut self, n: &str) {
        if m(n, &["marble", "ball"]) && self.room == 4 && self.has("marble") {
            self.remove("marble");
            self.set("billiard_passage");
            self.award(25);
            self.print("You roll the marble into a pocket. Something clicks beneath the table, and a panel in the wall slides open.");
            self.print("A stair descends into the cellar.");
        } else if m(n, &["bucket"]) && self.room == 140 && self.has("bucket") {
            self.remove("bucket");
            self.set("roboss_short");
            self.print("You hurl the bucket of water over the Roboss. Sparks fountain from its joints, and it retreats, twitching.");
        } else if m(n, &["coconut"]) && self.room == 87 && self.has("coconut") {
            self.remove("coconut");
            self.set("peg_hit_coconut");
            self.print("You throw the coconut at the wooden peg. It strikes but does not quite knock it free.");
        } else if m(n, &["stone"]) && self.room == 87 && self.has("stone") {
            self.remove("stone");
            self.set("bridge_down");
            self.set("bridge_lowered");
            self.award(10);
            self.print("You throw the stone at the peg. It spins free, the winch releases, and the bridge drops across the chasm.");
        } else if m(n, &["snow", "schnee"]) && self.room == 33 && self.has("snow") {
            self.remove("snow");
            self.set("snow_soft");
            self.print("You hurl the snow into the shaft. It settles in a soft drift far below.");
        } else if m(n, &["wrecking", "grapple", "hook"]) && self.room == 156 && (self.has("wrecking") || self.has("grapple")) {
            if !self.flag("sandbank_reached") {
                self.set("sandbank_reached");
                self.award(5);
            }
            self.print("You throw the grappling hook at the fallen trunk. It catches, and you can now cross to the sandbank.");
        } else if m(n, &["wrecking", "grapple", "hook"]) && self.room == 157 && (self.has("wrecking") || self.has("grapple")) {
            self.set("findling_anchor");
            self.print("You throw the grappling hook at the findling across the water. It holds.");
        } else if m(n, &["chair"]) && self.room == 205 && self.flag("glasses_on_rail") && !self.flag("glasses_broken") {
            self.set("glasses_broken");
            self.award(10);
            self.print("You seize the chair and hurl it at the balustrade. It strikes Khalin's glasses, which topple and shatter on the floor below.");
        } else if m(n, &["pit"]) && self.room == 184 && self.flag("pit_out") && !self.flag("pit_thrown") {
            self.set("pit_thrown");
            self.print("You take Pit from your shoulder and toss him into the black lake. He vanishes with a shriek.");
        } else if m(n, &["thorns", "thorn", "dornen"]) && self.room == 77 && self.flag("thorns_taken") {
            self.set("fence_ladder");
            self.award(10);
            self.print("You throw the thorn bush onto the palisade. Its branches catch and weave themselves into a ladder.");
        } else if m(n, &["thorns", "thorn", "dornen"]) && self.room == 77 {
            self.print("You have no thorns. The coconut could break some loose.");
        } else {
            self.print("You throw it, but nothing happens.");
        }
    }

    fn give(&mut self, n: &str) {
        if self.room == 134 && m(n, &["card", "karte"]) {
            if self.has("card_coded") {
                self.print("The cleaning robot scans the coded card and nods you through.");
                self.set("robo_passed");
            } else if self.has("card") {
                self.set("robo_ask");
                self.print("The robot takes the card, then asks: 'What was the first name of Harold Jugger's mother?' SAY <name>.");
            } else {
                self.print("You have no card.");
            }
        } else if self.room == 160 && m(n, &["elixir", "flask"]) && self.has("elixir") {
            self.remove("elixir");
            self.set("slave_recovering");
            self.award(10);
            self.print("You pour the elixir between the exhausted man's lips. Colour returns to his face.");
            self.print("He sits up, but he cannot work without his pickaxe. WAIT a moment, then GIVE PICKAXE.");
        } else if self.room == 160 && m(n, &["pickaxe"]) && self.has("pickaxe") && self.flag("slave_recovering") {
            self.remove("pickaxe");
            self.set("pickaxe_given");
            self.set("slave_helped");
            self.print("You give the man his pickaxe. He grips your hand and promises to fill your cart when you next ride.");
        } else if self.room == 80 && self.flag("admiral_met") {
            let mut given = 0;
            if m(n, &["salt", "salz"]) && self.has("salt") {
                self.remove("salt");
                self.set("gave_salt");
                given += 1;
            }
            if m(n, &["pepper", "pfeffer"]) && self.has("pepper") {
                self.remove("pepper");
                self.set("gave_pepper");
                given += 1;
            }
            if m(n, &["rosemary", "rosmarin"]) && self.has("rosemary") {
                self.remove("rosemary");
                self.set("gave_rosemary");
                given += 1;
            }
            if m(n, &["oregano"]) && self.has("oregano") {
                self.remove("oregano");
                self.set("gave_oregano");
                given += 1;
            }
            if m(n, &["cork", "korken", "maggi"]) && (self.has("cork") || self.flag("lovage_ready")) {
                if self.has("cork") {
                    self.remove("cork");
                }
                self.set("gave_lovage");
                given += 1;
            }
            if given == 0 {
                self.print("The admiral wants salt, pepper, rosemary, oregano and lovage - or a cork of Maggi.");
            } else if self.flag("gave_salt") && self.flag("gave_pepper") && self.flag("gave_rosemary") && self.flag("gave_oregano") && self.flag("gave_lovage") {
                self.add("champignon");
                self.award(50);
                self.print("Admiral Atterson tastes the five seasonings, sighs with pleasure, and hands you a fat champignon. 'Eat it with the mushroom. It is the antidote.'");
            } else {
                self.print("The admiral takes it. 'More. I need five flavours.'");
            }
        } else if self.room == 170 && m(n, &["shard", "splitter", "gem"]) && self.has_shard(6) {
            self.lose_shard(6);
            self.set("eye_working");
            self.award(100);
            self.print("You give the sixth splinter to the smith. He sets to work at the anvil, and the forge roars.");
        } else if self.room == 236 && m(n, &["sigma", "cell", "zelle"]) && self.has("sigma") {
            self.remove("sigma");
            self.set("kal_has_sigma");
            self.print("You hand Kal the sigma cell. He fits it into the chair, and the glass wall shivers.");
            self.print("'Three questions,' he says. 'Answer YES or NO.'");
        } else if self.room == 236 && m(n, &["eye"]) {
            self.print("Kal looks at the Eye and smiles sadly. 'Keep it. It was never mine.'");
        } else {
            self.print("Nothing happens.");
        }
    }

    fn ask(&mut self, n: &str) {
        if self.room == 80 && self.flag("admiral_met") {
            if m(n, &["mushroom", "pilz"]) {
                self.print("'The mushrooms on this side make you shrink. Never eat one alone.'");
            } else if m(n, &["blossom", "scent", "duft", "flower"]) {
                self.print("'The blossoms are a narcotic. They drop a man where he stands and send his soul back to the stone circle.'");
            } else if m(n, &["effect", "side", "wirkung"]) {
                self.print("'The shrinking is temporary - but permanent if you never take the antidote. You would dwindle until you vanished.'");
            } else if m(n, &["antidote", "gegenmittel", "champignon"]) {
                self.print("'A champignon grown on the far side of the jungle. Eat it with the mushroom.'");
            } else if m(n, &["stomach", "bauch", "ache"]) {
                self.print("'My stomach is ruined by this island. Would you do me a favour?' Say YES, then ASK ADMIRAL ABOUT FAVOUR.");
            } else if m(n, &["favour", "favor", "gefälligkeit", "quest"]) {
                self.set("admiral_quest");
                self.print("'Bring me salt, pepper, rosemary, oregano and lovage. Do that, and I will show you a safe path through the jungle.'");
                self.print("'If you cannot find lovage, Maggi seasoning in a cork will do.'");
            } else {
                self.print("'Ask me about mushrooms, blossoms, the antidote, or my stomach.'");
            }
        } else if self.room == 169 {
            if m(n, &["tell", "name", "man"]) {
                self.print("'Tell. My name is Tell. I am the man who took Kal's Eye.'");
                self.set("tell_met");
            } else if m(n, &["slave", "work"]) {
                self.print("'No. I am no slave. Not any more.'");
            } else {
                self.print("'Ask me my name.'");
            }
        } else if self.room == 170 {
            if m(n, &["problem", "sixth", "splitter", "shard"]) {
                self.print("'The Eye needs six splinters. You have brought five. The sixth lies somewhere in this mine.'");
            } else if m(n, &["eye", "auge", "forge"]) {
                self.set("smith_eye_asked");
                self.print("'Bring me the sixth splinter and I will set the Eye. Without it, the ring will not close.'");
                if !self.has("elixir") && !self.flag("elixir_given") {
                    self.set("elixir_given");
                    self.add("elixir");
                    self.print("He presses a small flask into your hand. 'Elixir. For the slave you will meet.'");
                }
            } else {
                self.print("'Ask me about the problem, or about the Eye.'");
            }
        } else if self.room == 182 && self.flag("pit_out") {
            self.pit_riddle();
        } else if self.room == 236 && self.flag("kal_released") {
            self.print("'Ask me nothing. Answer my three questions.'");
        } else if self.has("amulet") && m(n, &["pit"]) {
            self.print("Something in the amulet mutters. 'Open me in the black crypt,' says a tiny voice.");
        } else {
            self.print("There is no one here who will answer.");
        }
    }

    fn say(&mut self, n: &str) {
        if self.room == 124 && self.flag("boston_asked") && !self.flag("boston_site") {
            if m(n, &["yes", "ja", "crew", "repair"]) {
                self.set("boston_site");
                self.print("'Site number?' asks the intercom.");
            } else {
                self.print("'Just answer the question,' says the intercom.");
            }
        } else if self.room == 124 && self.flag("boston_site") {
            if n.contains('7') || n.contains("seven") {
                self.set("boston_entered");
                self.print("'Site 7. Come in.' The gate unlocks.");
            } else {
                self.print("'That is not a valid site number.'");
            }
        } else if self.room == 128 && !self.flag("balt_asked") {
            self.print("The intercom waits. Press the button first.");
        } else if self.room == 128 && self.flag("balt_asked") && m(n, &["museum", "historical"]) {
            self.set("museum_info");
            self.print("'The Historical Museum is in District 11. Take the SRT.'");
        } else if self.room == 128 && m(n, &["secret", "geheimwort", "password"]) {
            self.set("balt_secret");
            self.print("'The secret word is hidden in the music the man on duty listens to.'");
        } else if self.room == 128 && m(n, &["roborap", "roborep", "rap"]) {
            if self.flag("balt_secret") {
                self.set("balt_entered");
                self.print("'Roborap. Correct.' The door opens onto District 6.");
            } else {
                self.print("The intercom is silent. You have not been asked for a password yet.");
            }
        } else if self.room == 134 && self.flag("robo_ask") && m(n, &["martha"]) {
            self.remove("card");
            self.add("card_coded");
            self.unset("robo_ask");
            self.print("'Martha. Correct.' The robot punches a code into the card and returns it. You now have a coded code card.");
        } else if self.room == 137 && self.flag("terminal_mode") {
            if m(n, &["aurelius"]) {
                self.set("output_code");
                self.award(25);
                self.print("The screen fills with a three-digit number: 472. 'The museum will accept this output code.'");
            } else if m(n, &["lift", "elevator"]) {
                self.set("lift_info");
                self.print("'To leave the information service, insert the card and say the release word.'");
            } else if m(n, &["schnuckiputzi", "release", "exit"]) {
                self.print("You say the word. The lift doors open. Return to the SRT.");
                self.enter(135);
            } else if m(n, &["museum", "artefact", "artifact", "radiation", "srt", "boston", "infoservice", "catastrophe", "disaster"]) {
                self.print("The terminal displays a page of history: the six worlds, the Eye of Kal, and the catastrophe that scattered the splinters. One name recurs: the priest AURELIUS.");
            } else {
                self.print("'Keyword not found.'");
            }
        } else if self.room == 145 && self.flag("museum_prompt") {
            if n.contains("472") || m(n, &["code"]) {
                if self.flag("output_code") {
                    self.set("museum_shard_ready");
                    self.unset("museum_prompt");
                    self.print("The terminal hums. Something drops into the output slot.");
                    self.print("TAKE SHARD.");
                } else {
                    self.print("The terminal rejects the code. You need the output code from the info service.");
                }
            } else {
                self.print("The terminal waits for the three-digit output code.");
            }
        } else if self.room == 169 {
            if m(n, &["no", "nein"]) {
                self.set("tell_no");
                self.print("'No,' you say. 'I am no slave.'");
            } else if m(n, &["yes", "ja"]) && self.flag("tell_no") {
                self.set("tell_yes");
                self.print("'Yes. I am looking for someone.' The man folds his arms and waits for a name.");
            } else if m(n, &["tell"]) && self.flag("tell_yes") {
                self.set("tell_met");
                self.print("'Tell,' you say. The man starts. 'Then you know what I took. Come - the forge is east.'");
            } else if self.flag("tell_met") {
                self.print("'The forge is east,' says Tell.");
            } else {
                self.print("'Are you a slave?' the man asks. Answer NO or YES.");
            }
        } else if self.room == 205 && self.flag("khalin_question") && m(n, &["no", "nein", "not yet"]) {
            self.set("khalin_no");
            self.print("'No,' you say. Khalin sighs, sets her glasses on the balustrade, and turns away.");
            self.set("glasses_on_rail");
        } else if self.room == 236 && self.flag("kal_has_sigma") && m(n, &["yes", "ja"]) {
            let count = if self.flag("kal_yes2") { 2 } else if self.flag("kal_yes1") { 1 } else { 0 };
            if count >= 2 {
                self.set("kal_done");
                self.award(100);
                self.print("'Then the Eye is yours, and the door is open.' Kal rises. Beyond him a spaceship hums on its pad.");
                self.enter(238);
            } else {
                self.set(&format!("kal_yes{}", count + 1));
                self.print("'Yes,' you answer. Kal nods and asks again.");
            }
        } else if self.room == 26 && self.flag("labyrinth_known") {
            self.print("The stone circle does not answer.");
        } else if m(n, &["hello", "hallo"]) {
            self.print("Your voice echoes back from the stone.");
        } else {
            self.print(&format!("You say, \"{}\". Nothing answers.", n));
        }
    }

    pub fn answer_yes(&mut self) {
        if self.room == 4 && self.flag("billiard_asked") && !self.flag("billiard_answered") {
            self.set("billiard_answered");
            self.set("table_damaged");
            self.print("You take the shot. The cue tears the green cloth and the marble jumps the table. No points for that.");
        } else if self.room == 80 && self.flag("admiral_quest") {
            self.print("'Good. Then bring me the five flavours.'");
        } else if self.room == 124 {
            self.say("yes");
        } else if self.room == 128 {
            self.say("yes");
        } else if self.room == 236 {
            self.say("yes");
        } else {
            self.print("You nod. Nothing happens.");
        }
    }

    pub fn answer_no(&mut self) {
        if self.room == 4 && self.flag("billiard_asked") && !self.flag("billiard_answered") {
            self.set("billiard_answered");
            self.award(15);
            self.print("You think better of it and put the cue down. The table is unmarked.");
        } else if self.room == 205 && self.flag("khalin_question") {
            self.say("no");
        } else {
            self.print("You shake your head. Nothing happens.");
        }
    }

    fn climb(&mut self, n: &str) {
        if self.room == 1 && m(n, &["fireplace", "chimney", "kamin", "up"]) {
            self.enter(5);
            self.print("You duck under the mantel and climb into the soot-black fireplace.");
        } else if self.room == 5 && m(n, &["out", "down", "back"]) {
            self.enter(1);
        } else if self.room == 7 && m(n, &["bed", "bett"]) {
            self.set("bed_climbed");
            self.print("You climb onto the four-poster bed. The canopy is within reach.");
        } else if self.room == 9 && m(n, &["table", "tisch"]) {
            if self.flag("table_under_trapdoor") {
                self.set("on_table");
                self.print("You climb onto the table. The trapdoor bolt is now within reach.");
            } else {
                self.print("The table is not under the trapdoor.");
            }
        } else if self.room == 9 && m(n, &["trapdoor", "hatch", "ceiling", "luke"]) {
            if self.flag("trapdoor_open") {
                self.enter(12);
                self.print("You climb through the trapdoor into the attic.");
            } else if self.flag("on_table") && (self.has("grapple") || self.has("boathook")) {
                self.set("trapdoor_open");
                self.print("You hook the trapdoor bolt with the grappling hook and pull it back. The trapdoor swings open.");
                self.enter(12);
            } else {
                self.print("You cannot reach the bolt from here.");
            }
        } else if self.room == 70 && m(n, &["palm", "tree", "coconut"]) {
            self.enter(71);
            self.set("palm_climbed");
            self.set("palm_scratched");
            self.print("You shin up the palm, skinning your arms and legs on the rough bark.");
        } else if self.room == 71 && m(n, &["down", "palm", "tree"]) {
            self.enter(70);
        } else if self.room == 77 && m(n, &["up", "fence", "ladder"]) {
            if self.flag("fence_ladder") {
                self.enter(81);
                self.print("You climb the woven thorn ladder onto the palisade.");
            } else {
                self.print("The fence is too high and the thorns too sharp.");
            }
        } else if self.room == 81 && m(n, &["down", "inside", "in"]) {
            self.enter(83);
            self.print("You climb down on the inside, into the clearing before the jungle.");
        } else if self.room == 83 && m(n, &["fence", "over", "back"]) {
            if self.flag("fence_ladder") {
                self.enter(77);
                self.print("You climb back over the thorn ladder.");
            } else {
                self.print("There is no way over the fence from here.");
            }
        } else if self.room == 88 && m(n, &["up", "wall", "chimney", "cliff"]) {
            self.enter(89);
        } else if self.room == 89 && m(n, &["up", "chimney"]) {
            self.enter(90);
            self.print("You haul yourself up the chimney.");
        } else if self.room == 90 && m(n, &["up", "ledge"]) {
            self.enter(91);
            self.print("You climb onto the summit.");
        } else if self.room == 192 && m(n, &["up", "out", "skin"]) {
            self.enter(193);
        } else if self.room == 193 && m(n, &["up", "skin"]) {
            self.enter(194);
        } else if self.room == 205 && m(n, &["bench", "down", "floor"]) {
            self.print("You climb down from the table onto the bench, then to the floor.");
            self.enter(233);
        } else if self.room == 233 && m(n, &["up", "table", "pavilion"]) {
            self.enter(205);
        } else {
            self.print("You can't climb that.");
        }
    }

    fn jump(&mut self, n: &str) {
        if self.room == 66 && m(n, &["overboard", "over", "board", "sea", "water", "jump"]) {
            self.set("jumped_ship");
            self.award(25);
            self.print("You leap over the rail as the French guns thunder. The sea closes over your head.");
            self.enter(67);
        } else if self.room == 7 && (m(n, &["bed", "bett"]) || n.trim().is_empty()) {
            self.set("bed_jumped");
            self.print("You bounce on the bed until the canopy cloth works loose.");
        } else if self.room == 33 && m(n, &["down", "shaft", "jump"]) {
            if self.flag("rope_anchor") && self.flag("snow_soft") {
                self.print("You climb down the wrecking-ball rope until it runs out - then let go into the snow below.");
                self.enter(34);
            } else if self.flag("rope_anchor") {
                self.print("You climb down, but the bottom is hard rock. Throw snow down first.");
            } else {
                self.print("The shaft is far too deep to jump.");
            }
        } else if self.room == 175 && m(n, &["down", "chasm"]) {
            self.print("The chasm is bottomless. Do not.");
        } else {
            self.print("You jump. Nothing happens.");
        }
    }

    fn swim(&mut self, _n: &str) {
        if self.room == 67 {
            if self.flag("plank_reached") {
                self.enter(69);
                self.print("You strike out for the floating plank and haul yourself onto it.");
            } else {
                self.print("You tread water. A plank bobs nearby - DIVE and surface to get your bearings.");
            }
        } else if self.room == 184 {
            if !self.flag("pit_thrown") {
                self.print("The black water stirs. Something vast is down there - deal with Pit first.");
            } else if !self.flag("pit_gone") {
                self.print("The water still boils. Wait for Pit to finish his business.");
            } else {
                self.enter(185);
                self.print("You dive into the black lake.");
            }
        } else {
            self.print("You can't swim here.");
        }
    }

    fn dive(&mut self, _n: &str) {
        if self.room == 67 {
            if self.flag("plank_reached") {
                self.print("You duck under the water and surface again. The plank is closer.");
            } else {
                self.set("plank_reached");
                self.print("You dive beneath the wreckage and surface beside a broad wooden plank. CLIMB PLANK.");
            }
        } else if self.room == 184 && !self.flag("pit_thrown") {
            self.print("The black water stirs. Something vast is down there - deal with Pit first.");
        } else if self.room == 184 && !self.flag("pit_gone") {
            self.print("The water still boils. Wait for Pit to finish his business.");
        } else if self.room == 184 {
            self.enter(185);
            self.print("You dive into the lake.");
        } else if self.room == 185 {
            if self.has("vine_rope") || self.has("root") {
                self.enter(188);
                self.print("You follow the root vine down to the deepest part of the lake.");
            } else {
                self.enter(191);
                self.print("You swim down to the lake bed.");
            }
        } else if self.room == 190 {
            if self.has("vine_rope") || self.has("root") {
                self.enter(188);
                self.print("You follow the root vine down to the deepest part of the lake.");
            } else {
                self.enter(191);
                self.print("You swim down to the lake bed.");
            }
        } else {
            self.print("You can't dive here.");
        }
    }

    fn light(&mut self, n: &str) {
        if m(n, &["lighter", "feuerzeug"]) && self.has("lighter") {
            if self.flag("lighter_lit") {
                self.print("The lighter is already burning.");
            } else if self.flag("lighter_filled") {
                self.set("lighter_lit");
                self.print("You strike the lighter. A clean blue flame springs up.");
            } else {
                self.print("The lighter is dry. Fill it with petroleum first.");
            }
        } else if m(n, &["candle", "kerze"]) && self.has("candle") {
            if self.flag("candle_lit") {
                self.print("The candle is already lit.");
            } else if self.flag("lighter_lit") {
                self.set("candle_lit");
                self.print("You light the candle. It gives a steady, warm light.");
            } else {
                self.print("You need a flame first.");
            }
        } else if m(n, &["torch", "fackel"]) && self.has("torch") {
            if self.flag("torch_lit") {
                self.print("The torch is already burning.");
            } else if self.flag("lighter_lit") {
                self.set("torch_lit");
                self.print("You touch the lighter to the oil-soaked flag. The torch roars into flame.");
            } else {
                self.print("You need a flame first.");
            }
        } else {
            self.print("You can't light that.");
        }
    }

    fn eat(&mut self, n: &str) {
        if m(n, &["mushroom", "pilz"]) && self.has("mushroom") {
            if self.has("champignon") {
                self.remove("mushroom");
                self.remove("champignon");
                self.set("antidote_eaten");
                self.set("admirals_blessing");
                self.print("You eat the mushroom, then the champignon. The world stops shrinking and settles back to its proper size.");
            } else {
                self.print("You remember the admiral's warning: never eat a jungle mushroom without a champignon. You put it away.");
            }
        } else if m(n, &["champignon"]) && self.has("champignon") {
            self.remove("champignon");
            self.print("You eat the champignon. It tastes of garlic and rain.");
        } else if m(n, &["bananas", "bananen"]) && self.has("bananas") {
            self.print("You are not hungry, and the bananas have a job to do.");
        } else {
            self.print("You can't eat that.");
        }
    }

    fn dig(&mut self, _n: &str) {
        if self.room == 16 {
            if self.has("pickaxe") || self.flag("pickaxe_given") {
                self.set("cellar_plate_open");
                self.print("You swing the pickaxe. The floor rings hollow, and a stone plate lifts free.");
            } else {
                self.print("You need a pickaxe.");
            }
        } else if self.room == 190 {
            self.set("sand_dug");
            self.print("You dig in the sand and uncover a waterlogged piece of wood.");
        } else if self.room == 34 {
            self.set("snow_pile_searched");
            self.print("You dig through the snow pile. Something metal glints inside.");
        } else {
            self.print("You dig, but find nothing.");
        }
    }

    fn fill(&mut self, n: &str) {
        if m(n, &["lighter", "feuerzeug"]) && self.has("lighter") {
            if self.has("bottle") {
                self.remove("bottle");
                self.set("lighter_filled");
                self.print("You fill the lighter from the bottle of 1922 petroleum.");
            } else {
                self.print("You need petroleum.");
            }
        } else {
            self.print("You can't fill that.");
        }
    }

    fn pour(&mut self, n: &str) {
        if m(n, &["maggi"]) && self.has("bottle_maggi") {
            if self.has("cork") {
                self.remove("bottle_maggi");
                self.remove("cork");
                self.set("lovage_ready");
                self.print("You pour the last of the Maggi into the crown cork. Lovage, by another name.");
            } else {
                self.print("You need something to pour it into - a crown cork would do.");
            }
        } else {
            self.print("You pour it out. Nothing happens.");
        }
    }

    fn wear(&mut self, n: &str) {
        let items: &[(&str, &str)] = &[
            ("coat", "coat"), ("boots", "boots"), ("gloves", "gloves"),
            ("goggles", "goggles"), ("hood", "hood"),
        ];
        let mut wore = false;
        for (id, word) in items {
            if m(n, &[word]) && self.has(id) {
                self.set(&format!("worn_{id}"));
                self.print(&format!("You put on the {}.", item_name(id)));
                wore = true;
            }
        }
        if m(n, &["clothes", "jacket", "pete"]) && self.flag("pete_clothes") {
            self.set("pete_worn");
            self.print("You put on Pete's salt-stained clothes.");
            wore = true;
        }
        if m(n, &["eyepatch"]) && self.flag("pete_patch") {
            self.set("pete_patch_worn");
            self.print("You tie on Pete's eyepatch.");
            wore = true;
        }
        if !wore {
            self.print("You can't wear that.");
            return;
        }
        if self.flag("worn_coat") && self.flag("worn_boots") && self.flag("worn_gloves") && self.flag("worn_goggles") && self.flag("worn_hood") {
            self.set("cold_protected");
            self.print("Wrapped in fur from head to foot, you are ready for the ice.");
        }
    }

    fn remove_item(&mut self, n: &str) {
        let items: &[(&str, &str)] = &[
            ("coat", "coat"), ("boots", "boots"), ("gloves", "gloves"),
            ("goggles", "goggles"), ("hood", "hood"), ("clothes", "clothes"), ("eyepatch", "eyepatch"),
        ];
        for (id, word) in items {
            if m(n, &[word]) {
                let f = format!("worn_{id}");
                if self.flag(&f) {
                    self.unset(&f);
                    self.print(&format!("You take off the {}.", item_name(id)));
                    return;
                }
            }
        }
        self.print("You aren't wearing that.");
    }

    fn destroy(&mut self, n: &str) {
        if m(n, &["web", "netz", "spinnennetz"]) && self.room == 8 {
            if self.flag("fly_given") || self.flag("spider_on_sponge") || self.flag("web_destroyed") {
                self.set("web_destroyed");
                self.print("You tear the web away. The toothbrush is free.");
            } else {
                self.print("The spider is still on guard.");
            }
        } else if m(n, &["box", "plastic", "kasten"]) && self.room == 122 {
            if self.has("hammer") {
                self.set("plastic_broken");
                self.set("hover_damaged");
                self.print("You smash the plastic box with the hammer. The hovercraft's maintenance unit is wrecked. A fuse lies among the fragments.");
            } else {
                self.print("You need a hammer.");
            }
        } else if m(n, &["chest", "sea chest"]) && self.room == 74 {
            if self.flag("chest_pulled") {
                self.set("chest_broken");
                self.set("chest_lid_open");
                self.add("pepper");
                self.print("You break the chest open. A packet of black powder spills out - pepper, not gunpowder.");
            } else {
                self.print("The chest is still in the water.");
            }
        } else if m(n, &["thorns", "dornen", "bush"]) && self.room == 77 {
            if self.has("coconut") {
                self.set("thorns_taken");
                self.print("You smash the thorn bush with the coconut. A bundle of thorny branches comes free - THROW THORNS ON FENCE.");
            } else {
                self.print("You need something hard. The coconut would do.");
            }
        } else if m(n, &["thread", "gewinde"]) && self.room == 122 {
            self.print("You need the tongs.");
        } else {
            self.print("You can't destroy that.");
        }
    }

    fn wait(&mut self) {
        let room = self.room;
        match room {
            9 => {
                if self.flag("jar_on_floor") && self.flag("sugar_in_jar") && !self.flag("fly_in_jar") {
                    self.set("fly_in_jar");
                    self.print("The fly buzzes down, settles on the sugar, and crawls into the jar. CLOSE JAR - quickly!");
                } else if self.flag("fly_in_jar") {
                    self.print("The fly buzzes inside the jar.");
                } else {
                    self.print("You wait. The house creaks.");
                }
            }
            66 => {
                if !self.flag("french_arrived") {
                    self.set("french_arrived");
                    self.print("A cry from the masthead: French sail! The gun crew scatters. A powder charge is left within reach. TAKE POWDER.");
                } else if !self.flag("cannon_loaded") && !self.has("powder") {
                    self.print("The French ships close in. Ram the powder into the cannon!");
                } else {
                    self.print("The cannon is loaded. Pete and Willy are already going over the side - JUMP OVERBOARD!");
                }
            }
            69 => {
                if self.flag("sail_rigged") && self.flag("mast_rigged") {
                    let n = if self.flag("sail_wait3") { 3 } else if self.flag("sail_wait2") { 2 } else if self.flag("sail_wait1") { 1 } else { 0 };
                    if n >= 2 {
                        self.award(25);
                        self.print("The wind fills the sail. The plank runs onto a sandy shore.");
                        self.enter(70);
                    } else {
                        self.set(&format!("sail_wait{}", n + 1));
                        self.print("The sail flaps, then fills. The island grows closer.");
                    }
                } else {
                    self.print("The plank drifts. It needs a sail.");
                }
            }
            139 => {
                if self.flag("fountain_on") && !self.flag("roboss_came") {
                    self.set("roboss_came");
                    self.print("The Roboss trundles over, muttering, and reaches for the fountain control.");
                } else if self.flag("roboss_came") && !self.flag("roboss_off") {
                    self.set("roboss_off");
                    self.set("bucket_full");
                    self.print("The Roboss shuts off the fountain and turns away. The bucket is full of water. TAKE BUCKET.");
                } else {
                    self.print("Robots bustle across the square.");
                }
            }
            163 => {
                if !self.flag("cart_filled") {
                    self.set("cart_filled");
                    if self.flag("slave_helped") {
                        self.print("The freed slave shovels ore into the cart and grins. The brakes release.");
                    } else {
                        self.print("Ore pours into the cart. The brakes release.");
                    }
                } else {
                    if !self.flag("ore_station_awarded") {
                        self.set("ore_station_awarded");
                        self.award(10);
                    }
                    if self.flag("slave_helped") {
                        self.set("sixth_shard_ready");
                        self.print("The cart rattles east into the tunnel and stops at the great ore station. The freed slave has filled it with fresh ore.");
                    } else {
                        self.print("The cart rattles east into the tunnel and stops at the great ore station. You climb out.");
                    }
                    self.enter(165);
                }
            }
            170 => {
                if self.flag("eye_working") && !self.flag("eye_ready") {
                    self.set("eye_ready");
                    self.print("The smith quenches the ring in a trough. The six splinters blaze, and he holds up the Eye of Kal. TAKE EYE.");
                } else if self.flag("eye_ready") {
                    self.print("The smith watches you, waiting.");
                } else {
                    self.print("The forge roars.");
                }
            }
            178 => {
                if !self.flag("beast_gone") {
                    let n = if self.flag("beast_wait2") { 2 } else if self.flag("beast_wait1") { 1 } else { 0 };
                    if n >= 1 {
                        self.set("beast_gone");
                        self.award(50);
                        self.print("The beast gives a roar of frustration, drops something that clatters on the steps, and vanishes into the dark.");
                    } else {
                        self.set(&format!("beast_wait{}", n + 1));
                        self.print("You pace up and down. The beast growls, louder each time.");
                    }
                } else {
                    self.print("The stairs are quiet now.");
                }
            }
            184 => {
                if self.flag("pit_thrown") && !self.flag("pit_resurfaced") {
                    let n = if self.flag("pit_wait1") { 1 } else { 0 };
                    if n >= 1 {
                        self.set("pit_resurfaced");
                        self.set("pit_gone");
                        self.print("Pit surfaces, splutters, and vanishes with a rude gesture. The lake is safe now - DIVE.");
                    } else {
                        self.set("pit_wait1");
                        self.print("The water boils. Something vast rolls over in the depths.");
                    }
                } else {
                    self.print("The black water laps at the shore.");
                }
            }
            195 => {
                if self.flag("kathaura_trapped") && self.flag("sluice_filled") && !self.flag("kathaura_dead") {
                    self.set("kathaura_dead");
                    self.award(25);
                    self.print("The monster surges out of the tunnel. Kathaura's bellow ends in a gurgle. When the water settles, the giant is gone.");
                } else if self.flag("sluice_filled") && self.flag("gate_open") && self.flag("grille_open") && !self.flag("monster_in_sluice") {
                    self.set("monster_in_sluice");
                    self.print("A second Bool is swept through the tunnel into the sluice. It thrashes against the grille.");
                } else if self.flag("kathaura_dead") {
                    self.print("The sluice is quiet. Read the plaque, or go north.");
                } else {
                    self.print("Water gurgles in the pipes.");
                }
            }
            202 => {
                if self.flag("blackgate_open") {
                    self.print("The gate stands open. Khalin watches from her window.");
                } else if self.flag("khalin_called") {
                    self.set("blackgate_open");
                    self.print("Khalin leans down, enormous and curious, and draws the gate open with one finger.");
                } else {
                    self.print("Rain falls on the fortress wall. A button waits beside the gate.");
                }
            }
            205 => {
                if self.flag("glasses_broken") {
                    self.print("Khalin's chair stands empty. The broken glasses glitter on the floor far below.");
                } else if self.flag("glasses_on_rail") {
                    self.print("Khalin has turned her back. Her glasses lie on the balustrade. THROW CHAIR AT GLASSES!");
                } else if self.flag("khalin_no") {
                    self.print("Khalin sighs and moves to the balustrade.");
                } else if self.flag("khalin_question") {
                    self.print("Khalin is waiting for your answer. SAY NO.");
                } else {
                    self.set("khalin_question");
                    self.print("A shadow falls over the pavilion. Khalin's voice booms: 'HAVE YOU FOUND THE EYE OF KAL?' Answer NO - a giant must not know you have it.");
                }
            }
            236 => {
                if self.flag("kal_released") && !self.flag("kal_has_sigma") {
                    self.print("Kal points at a socket in his chair. 'A sigma cell. Fetch me one.'");
                } else if self.flag("kal_has_sigma") {
                    self.print("Kal waits for your answers. SAY YES.");
                } else {
                    self.print("The glass wall hums.");
                }
            }
            _ => self.print("Time passes."),
        }
    }

    fn stand(&mut self, n: &str) {
        if self.room == 26 {
            for c in ["white", "yellow", "green", "red", "blue", "black"] {
                if n.contains(c) {
                    self.field = Some(match c {
                        "white" => "white",
                        "yellow" => "yellow",
                        "green" => "green",
                        "red" => "red",
                        "blue" => "blue",
                        _ => "black",
                    });
                    self.print(&format!("You stand on the {c} field."));
                    return;
                }
            }
            self.print("Stand on which field? White, yellow, green, red, blue or black.");
        } else {
            self.print("You stand still.");
        }
    }

    fn limp(&mut self, _n: &str) {
        if self.room == 95 {
            if self.flag("pete_clothes") && self.flag("pete_patch") && self.flag("palm_scratched") {
                self.set("pete_disguise");
                self.print("You put on Pete's clothes and eyepatch, hunch your shoulder and limp north, dragging your leg exactly as he did.");
                self.enter(96);
            } else {
                self.print("You limp, but the disguise is not convincing enough. You need Pete's clothes, his eyepatch, and real wounds.");
            }
        } else {
            self.print("You limp a few steps. Nothing happens.");
        }
    }

    fn bury(&mut self, n: &str) {
        if self.room == 73 && m(n, &["willy", "body", "corpse", ""]) {
            if self.flag("willy_buried") {
                self.print("Willy's grave is already marked with stones.");
            } else {
                self.set("willy_buried");
                self.award(10);
                self.print("You dig a grave in the sand with your hands and lay Willy to rest. You pile stones over him.");
            }
        } else {
            self.print("There is nothing here to bury.");
        }
    }

    fn drive(&mut self, n: &str) {
        if self.room == 69 {
            if m(n, &["pole", "mast", "stange"]) && !self.has("pole") {
                self.set("at_pole");
                self.print("You paddle the plank over to the floating pole.");
            } else if m(n, &["sail", "segel"]) && !self.has("sail") {
                self.set("at_sail");
                self.print("You paddle the plank over to the floating sailcloth.");
            } else {
                self.print("You paddle the plank. It goes nowhere in particular.");
            }
        } else {
            self.print("You can't drive here.");
        }
    }

    fn insert(&mut self, n: &str) {
        if self.room == 136 {
            if self.has("card_coded") {
                self.print("The coded card slides into the slot. The lift opens onto the terminal hall.");
                self.enter(137);
            } else if self.has("card") {
                self.print("The slot flashes red. The card has no code.");
            } else {
                self.print("You have no card.");
            }
        } else if (self.room == 234 || self.room == 235) && m(n, &["key"]) && self.has("kal_key") {
            self.remove("kal_key");
            self.set("kal_switch_ready");
            self.print("You insert the key into the switch. Somewhere a machine begins to turn.");
        } else if self.room == 24 && m(n, &["handle"]) {
            if self.has("handle") {
                self.remove("handle");
                self.set("handle_inserted");
                self.print("You insert the handle into the slot.");
            } else {
                self.print("You have no handle.");
            }
        } else if self.room == 32 && m(n, &["shard", "splitter"]) {
            self.place("shard in socket");
        } else {
            self.print("Inserting that has no effect.");
        }
    }

    fn hold(&mut self, n: &str) {
        if self.room == 188 && m(n, &["cube", "würfel", "wuerfel"]) {
            self.set("cube_charged");
            self.print("You turn the cube into the light beam. It drinks the light and begins to glow.");
        } else if self.room == 188 && m(n, &["scale", "schuppe"]) {
            self.place("scale in light");
        } else if m(n, &["breath"]) {
            self.print("You hold your breath.");
        } else {
            self.print("You hold it. Nothing happens.");
        }
    }

    fn show(&mut self, n: &str) {
        if self.room == 169 && m(n, &["shard", "splitter", "gem", "eye"]) {
            self.set("tell_shards");
            self.print("You show Tell the five splinters. 'Five,' he says. 'The sixth is still in the mine. We will need it before the Eye can be made.'");
        } else if self.room == 236 && m(n, &["eye"]) {
            self.print("You hold up the Eye. Kal regards it with something like grief.");
        } else {
            self.print("No one is interested.");
        }
    }

    fn knock(&mut self) {
        if self.room == 236 {
            if self.flag("kal_released") {
                self.print("The glass wall is open.");
            } else {
                self.set("kal_seen");
                self.print("You knock on the glass. The seated man turns. The nameplate reads KAL. PRESS BUTTON to release him.");
            }
        } else if self.room == 1 {
            self.open("door");
        } else {
            self.print("You knock. No answer.");
        }
    }

    fn extinguish(&mut self, n: &str) {
        if m(n, &["torch"]) {
            self.unset("torch_lit");
            self.print("You smother the torch.");
        } else if m(n, &["candle"]) {
            self.unset("candle_lit");
            self.print("You pinch out the candle.");
        } else if m(n, &["lighter"]) {
            self.unset("lighter_lit");
            self.print("You close the lighter.");
        } else {
            self.print("There is no flame to put out.");
        }
    }

    // ------------------------------------------------------------------
    // Special mechanisms
    // ------------------------------------------------------------------

    fn srt_panel(&mut self, n: &str) {
        let n = n.trim();
        let step = |g: &Game| -> i32 {
            (1..=6).filter(|i| g.flag(&format!("srtseq{i}"))).count() as i32
        };
        if n == "10" || n.contains("ten") && !n.contains("fifteen") {
            self.print("The SRT glides to the city centre.");
            self.enter(139);
        } else if n == "14" || n.contains("fourteen") {
            if self.flag("museum_info") || self.flag("balt_entered") {
                self.print("The SRT carries you to District 14 and opens into a lift.");
                self.enter(136);
            } else {
                self.print("The panel rejects the destination. You do not know where to go yet.");
            }
        } else if n == "6" || n == "six" {
            self.set("srtseq1");
            self.print("You press 6. The panel waits for the next number.");
        } else if (n == "12" || n.contains("twelve")) && step(self) == 1 {
            self.set("srtseq2");
            self.print("You press 12. The panel waits.");
        } else if (n == "5" || n.contains("five")) && step(self) == 2 {
            self.set("srtseq3");
            self.print("You press 5. The panel waits.");
        } else if (n == "9" || n.contains("nine")) && step(self) == 3 {
            self.set("srtseq4");
            self.print("You press 9. The panel waits.");
        } else if (n == "15" || n.contains("fifteen")) && step(self) == 4 {
            self.set("srtseq5");
            self.print("You press 15. The panel waits.");
        } else if (n == "13" || n.contains("thirteen")) && step(self) == 5 {
            self.set("srtseq6");
            self.print("You press 13. The panel waits.");
        } else if (n == "11" || n.contains("eleven")) && step(self) == 6 {
            for i in 1..=6 { self.unset(&format!("srtseq{i}")); }
            self.print("You press 11. The SRT runs a long way through the dark and stops at the Historical Museum.");
            self.award(25);
            self.enter(143);
        } else {
            self.print("The panel flashes. That is not the right number yet.");
        }
    }

    fn stone_circle(&mut self, n: &str) {
        let field = match self.field {
            Some(f) => f,
            None => {
                self.print("Stand on a coloured field first: STAND ON WHITE, RED, GREEN, YELLOW or BLACK.");
                return;
            }
        };
        let numeral = if n.contains("iii") || n.contains("3") || n.contains("three") {
            3
        } else if n.contains("vi") || n.contains("6") || n.contains("six") {
            6
        } else if n.contains("iv") || n.contains("4") || n.contains("four") {
            4
        } else if n.contains("v") || n.contains("5") || n.contains("five") {
            5
        } else if n.contains("ii") || n.contains("2") || n.contains("two") {
            2
        } else if n.contains("i") || n.contains("1") || n.contains("one") {
            1
        } else {
            0
        };
        if numeral == 0 {
            self.print("Press which block? I to VI.");
            return;
        }
        let world = match (field, numeral) {
            ("white", 1) => Some((27u16, "ice")),
            ("yellow", 2) => Some((146, "prehistory")),
            ("red", 3) => Some((120, "usa")),
            ("blue", 4) => Some((65, "ship")),
            ("green", 5) => Some((65, "ship")),
            ("black", 6) => Some((160, "mine")),
            _ => None,
        };
        let Some((room, name)) = world else {
            self.print("The block sinks, but nothing happens. That is not the right field and numeral.");
            return;
        };
        if name == "mine" && self.shards() < 5 {
            self.print("The black block will not move. You feel that five splinters are needed to open the last world.");
            return;
        }
        if name == "mine" && !(self.flag("world_usa") && self.flag("world_ice") && self.flag("world_ship") && self.flag("world_prehistory")) {
            self.print("The black block refuses you. The other four worlds must be finished first.");
            return;
        }
        self.set(&format!("world_{name}"));
        if !self.flag(&format!("travel_{name}")) {
            self.set(&format!("travel_{name}"));
            if name == "usa" {
                self.award(50);
            }
        }
        self.field = None;
        self.enter(room);
        self.print("The stone circle dissolves around you.");
    }

    fn intercom_boston(&mut self) {
        if self.flag("boston_entered") {
            self.print("The gate stands open.");
            return;
        }
        if self.flag("boston_site") {
            self.print("'Site number?' asks the intercom.");
        } else if self.flag("boston_asked") {
            self.print("'Are you with the repair crew? Answer YES or NO.'");
        } else {
            self.set("boston_asked");
            self.print("You press the button. A voice crackles: 'Boston depot. Are you with the repair crew?'");
        }
    }

    fn intercom_baltimore(&mut self) {
        if self.flag("balt_entered") {
            self.print("The door stands open.");
            return;
        }
        if self.flag("balt_secret") {
            self.print("The intercom waits for the secret word.");
        } else if self.flag("balt_asked") {
            self.print("'Where do you want to go?' it asks.");
        } else {
            self.set("balt_asked");
            self.print("You press the button. 'Baltimore service entrance. Where do you want to go?'");
        }
    }

    fn kal_button(&mut self) {
        if self.flag("kal_released") {
            self.print("Kal is already free.");
        } else if self.flag("kal_seen") {
            self.award(100);
            self.set("kal_released");
            self.set("kal_question_count");
            self.print("You press the button. The glass wall rises, and Kal turns to face you.");
            self.print("'You have the Eye,' he says. 'Then bring me a sigma cell, and I will tell you what it is for.'");
        } else {
            self.print("A man sits behind the glass with his back to you. Knock first.");
        }
    }

    fn pull_drawbar(&mut self) {
        if !self.flag("kathaura_seen") {
            self.set("kathaura_seen");
            self.set("draw1");
            self.print("You pull the drawbar. A giant in black armour rises from the floor: Kathaura. He plants himself across the north way.");
            return;
        }
        let n = if self.flag("draw3") { 3 } else if self.flag("draw2") { 2 } else if self.flag("draw1") { 1 } else { 0 };
        if n >= 3 {
            if self.flag("cable195_cut") && self.flag("cable198_cut") && self.flag("monster_in_sluice") {
                self.set("kathaura_lured");
                self.set("grille_open");
                self.print("You pull the drawbar a fourth time. The grille grinds open. Kathaura roars and lumbers after you - GO SOUTH, THEN WEST, AND CLOSE THE GATE!");
            } else {
                self.print("The drawbar moves, but the grille is dead. The control cables must be cut first, and the monster must be in the sluice.");
            }
        } else {
            self.set(&format!("draw{}", n + 1));
            self.print("You pull the drawbar. Kathaura takes a step closer.");
        }
    }

    fn pit_riddle(&mut self) {
        const RIDDLES: [&str; 11] = [
            "Pit whispers: 'I am small, striped and angry, and I sting. Which way does the wasp fly?'",
            "Pit whispers: 'Amundsen sought the pole of ice. Which way did he go?'",
            "Pit whispers: 'Sailors steer by a cross in the southern sky. Which way?'",
            "Pit whispers: 'Where does the sun rise?'",
            "Pit whispers: 'The way down is sometimes the only way on.'",
            "Pit whispers: 'Remarque wrote that nothing is new in which direction?'",
            "Pit whispers: 'The cold star of the north.'",
            "Pit whispers: 'Where is Venus in the evening sky?'",
            "Pit whispers: 'Jules Verne travelled to the centre of the Earth, through a volcano. Where is Iceland?'",
            "Pit whispers: 'Penguins south, bears north, cowboys west, veiled women east - and the dead below. Which way for the dead?'",
            "Pit whispers: 'Eggs, rabbits and resurrection. Which way does Easter point?'",
        ];
        let i = (self.lab_step as usize).min(10);
        self.print(RIDDLES[i]);
        self.print("Then MOVE in the direction Pit means.");
    }

    pub fn labyrinth_move(&mut self, dir: &str) {
        const STEPS: [&str; 11] = [
            "west", "south", "south", "east", "down", "west", "north", "west", "north", "up", "east",
        ];
        let i = self.lab_step as usize;
        if i >= 11 {
            self.enter(184);
            return;
        }
        if dir == STEPS[i] {
            self.lab_step += 1;
            if self.lab_step >= 11 {
                self.print("The last black door opens. Cold, still water lies beyond.");
                self.enter(184);
            } else {
                self.print("Pit cackles. 'That way!' You step through into another six-sided chamber.");
            }
        } else {
            self.print("A wrong turn. The labyrinth twists you back to the first chamber.");
            self.lab_step = 0;
        }
    }
}
