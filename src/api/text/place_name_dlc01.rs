use std::{collections::HashMap, sync::LazyLock};

pub static PLACE_NAME_DLC01: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| {
    HashMap::from([
		(20000,"Belurat, Tower Settlement"),
		(20001,""),
		(20002,""),
		(20003,""),
		(20004,""),
		(20005,""),
		(20006,""),
		(20007,""),
		(20008,""),
		(20009,""),
		(20010,"Enir-Ilim"),
		(21000,"Shadow Keep"),
		(21001,"Shadow Keep, Church District"),
		(21002,""),
		(21003,""),
		(21004,""),
		(21005,""),
		(21006,""),
		(21007,""),
		(21008,""),
		(21009,""),
		(21010,"Specimen Storehouse"),
		(22000,"Stone Coffin Fissure"),
		(28000,"Midra's Manse"),
		(40000,"Fog Rift Catacombs"),
		(40001,""),
		(40002,""),
		(40003,""),
		(40004,""),
		(40005,""),
		(40006,""),
		(40007,""),
		(40008,""),
		(40009,""),
		(40010,"Scorpion River Catacombs"),
		(40011,""),
		(40012,""),
		(40013,""),
		(40014,""),
		(40015,""),
		(40016,""),
		(40017,""),
		(40018,""),
		(40019,""),
		(40020,"Darklight Catacombs"),
		(41000,"Belurat Gaol"),
		(41001,""),
		(41002,""),
		(41003,""),
		(41004,""),
		(41005,""),
		(41006,""),
		(41007,""),
		(41008,""),
		(41009,""),
		(41010,"Bonny Gaol"),
		(41011,""),
		(41012,""),
		(41013,""),
		(41014,""),
		(41015,""),
		(41016,""),
		(41017,""),
		(41018,""),
		(41019,""),
		(41020,"Lamenter's Gaol"),
		(42000,"Ruined Forge Lava Intake"),
		(42020,"Ruined Forge of Starfall Past"),
		(42021,""),
		(42022,""),
		(42023,""),
		(42024,""),
		(42025,""),
		(42026,""),
		(42027,""),
		(42028,""),
		(42029,""),
		(42030,"Taylew's Ruined Forge"),
		(43000,"Rivermouth Cave"),
		(43001,""),
		(43002,""),
		(43003,""),
		(43004,""),
		(43005,""),
		(43006,""),
		(43007,""),
		(43008,""),
		(43009,""),
		(43010,"Dragon's Pit"),
		(68000,"Gravesite Plain"),
		(68200,"Castle Ensis"),
		(68300,"Cerulean Coast"),
		(68400,"Charo's Hidden Grave"),
		(68500,"Jagged Peak"),
		(68501,"Foot of the Jagged Peak"),
		(68600,"Abyssal Woods"),
		(69000,"Scadu Altus"),
		(69001,"Rauh Base"),
		(69200,"Scaduview"),
		(69400,"Ancient Ruins of Rauh"),
		(200000,"Theatre of the Divine Beast"),
		(200001,"Belurat, Tower Settlement"),
		(200002,"Small Private Altar"),
		(200003,"Stagefront"),
		(200100,"Gate of Divinity"),
		(200101,""),
		(200102,"Enir-Ilim: Outer Wall"),
		(200103,"First Rise"),
		(200104,"Spiral Rise"),
		(200105,"Cleansing Chamber Anteroom"),
		(200106,"Divine Gate Front Staircase"),
		(210001,"Main Gate Plaza"),
		(210002,"Shadow Keep Main Gate"),
		(210003,""),
		(210004,""),
		(210005,""),
		(210006,"Church District Entrance"),
		(210007,"Sunken Chapel"),
		(210008,"Tree-Worship Passage"),
		(210009,"Tree-Worship Sanctum"),
		(210100,"Messmer's Dark Chamber"),
		(210101,"Storehouse, First Floor"),
		(210102,"Storehouse, Fourth Floor"),
		(210103,"Storehouse, Seventh Floor"),
		(210104,"Dark Chamber Entrance"),
		(210105,""),
		(210106,"Storehouse, Back Section"),
		(210107,"Storehouse, Loft"),
		(210201,"West Rampart"),
		(220000,"Garden of Deep Purple"),
		(220001,"Stone Coffin Fissure"),
		(220002,"Fissure Cross"),
		(220003,"Fissure Waypoint"),
		(220004,"Fissure Depths"),
		(250000,"Finger Birthing Grounds"),
		(280000,"Discussion Chamber"),
		(280001,"Manse Hall"),
		(280002,"Midra's Library"),
		(280003,"Second Floor Chamber"),
		(400000,"Fog Rift Catacombs"),
		(400100,"Scorpion River Catacombs"),
		(400200,"Darklight Catacombs"),
		(410000,"Belurat Gaol"),
		(410100,"Bonny Gaol"),
		(410200,"Lamenter's Gaol"),
		(420000,"Ruined Forge Lava Intake"),
		(420200,"Ruined Forge of Starfall Past"),
		(420300,"Taylew's Ruined Forge"),
		(430000,"Rivermouth Cave"),
		(430100,"Dragon's Pit"),
		(430101,"Dragon's Pit Terminus"),
		(680000,"Gravesite Plain"),
		(680001,"Scorched Ruins"),
		(680002,"Three-Path Cross"),
		(680003,"Main Gate Cross"),
		(680004,"Cliffroad Terminus"),
		(680005,"Greatbridge, North"),
		(681000,"Ellac River Cave"),
		(681001,"Pillar Path Cross"),
		(681002,"Pillar Path Waypoint"),
		(681003,"Castle Front"),
		(682000,"Castle Ensis Checkpoint"),
		(682001,"Castle-Lord's Chamber"),
		(682002,"Ensis Moongazing Grounds"),
		(683000,"Ellac River Downstream"),
		(683001,"Cerulean Coast"),
		(683002,"Cerulean Coast West"),
		(683003,"The Fissure"),
		(683004,"Finger Ruins of Rhia"),
		(683005,"Cerulean Coast Cross"),
		(684000,"Grand Altar of Dragon Communion"),
		(684001,"Charo's Hidden Grave"),
		(685000,"Foot of the Jagged Peak"),
		(685001,"Jagged Peak Mountainside"),
		(685002,"Jagged Peak Summit"),
		(685003,"Rest of the Dread Dragon"),
		(686000,"Abyssal Woods"),
		(686001,"Divided Falls"),
		(686002,"Forsaken Graveyard"),
		(686003,"Woodland Trail"),
		(686004,"Church Ruins"),
		(690000,"Highroad Cross"),
		(690001,""),
		(690002,"Moorth Ruins"),
		(690003,"Bonny Village"),
		(690004,"Bridge Leading to the Village"),
		(690005,"Church District Highroad"),
		(690006,"Cathedral of Manus Metyr"),
		(690007,"Scadu Altus, West"),
		(690008,"Moorth Highway, South"),
		(690009,"Fort of Reprimand"),
		(690010,"Behind the Fort of Reprimand"),
		(690011,"Scaduview Cross"),
		(690012,"Ancient Ruins Base"),
		(690013,"Temple Town Ruins"),
		(690014,"Ravine North"),
		(690015,""),
		(690016,"Castle Watering Hole"),
		(690017,"Recluses' River Upstream"),
		(690018,"Recluses' River Downstream"),
		(692000,"Scaduview"),
		(692001,"Shadow Keep, Back Gate"),
		(693000,"Hinterland"),
		(693001,"Fingerstone Hill"),
		(693002,"Hinterland Bridge"),
		(694000,"Viaduct Minor Tower"),
		(694001,"Rauh Ancient Ruins, East"),
		(694002,"Rauh Ancient Ruins, West"),
		(694003,"Church of the Bud, Main Entrance"),
		(694004,"Ancient Ruins, Grand Stairway"),
		(694005,"Church of the Bud"),
		(696000,"Scadutree Base"),
		(6800800,"Miquella's Cross"),
		(6800801,"Miquella's Cross"),
		(6801000,"Belurat, Tower Settlement"),
		(6804400,"Abandoned Ailing Village"),
		(6804700,"Church of Consolation"),
		(6804701,"Church of Benediction"),
		(6805000,"Scorched Ruins"),
		(6805100,"Prospect Town"),
		(6805600,"Western Nameless Mausoleum"),
		(6805900,"Run-Down Traveler's Rest"),
		(6805901,"Artist's Shack"),
		(6806000,"Ellac Greatbridge"),
		(6810800,"Miquella's Cross"),
		(6815700,"Suppressing Pillar"),
		(6815901,"Elder's Hovel"),
		(6820800,"Miquella's Cross"),
		(6824100,"Castle Ensis"),
		(6830000,"Finger Ruins of Rhia"),
		(6830800,"Miquella's Cross"),
		(6831700,"Stone Coffin Fissure"),
		(6835500,"Finger Ruins of Rhia"),
		(6835600,"Southern Nameless Mausoleum"),
		(6835900,"Finger-Weaver's Hovel"),
		(6844900,"Grand Altar of Dragon Communion"),
		(6861800,"Midra's Manse"),
		(6864700,"Abandoned Church"),
		(6870800,"Miquella's Cross"),
		(6880800,"Miquella's Cross"),
		(6890800,"Miquella's Cross"),
		(6900000,"Finger Ruins of Miyr"),
		(6900800,"Miquella's Cross"),
		(6900801,"Miquella's Cross"),
		(6900802,"Miquella's Cross"),
		(6901300,"Shadow Keep"),
		(6904200,"Fog Rift Fort"),
		(6904300,"Fort of Reprimand"),
		(6904700,"Church of the Crusade"),
		(6904800,"Cathedral of Manus Metyr"),
		(6905200,"Temple Town Ruins"),
		(6905300,"Ruins of Unte"),
		(6905400,"Moorth Ruins"),
		(6905500,"Finger Ruins of Miyr"),
		(6905600,"Northern Nameless Mausoleum"),
		(6905601,"Eastern Nameless Mausoleum"),
		(6905800,"Rabbath's Rise"),
		(6905900,"Whipping Hut"),
		(6906100,"Village of Flies"),
		(6906800,"Bonny Village"),
		(6920000,"Scaduview"),
		(6920001,"Scadutree Base"),
		(6924000,"Scadutree Chalice"),
		(6925900,"Albinauric's Shack"),
		(6930000,"Finger Ruins of Dheo"),
		(6934600,"Shaman Village"),
		(6935500,"Finger Ruins of Dheo"),
		(6940800,"Miquella's Cross"),
		(6941100,"Enir-Ilim"),
		(6990800,"Miquella's Cross"),
		(8020000,"Realm of Shadow"),
		(10670100,"Witchbane Ruins"),
		(10670101,"The First Step"),
		(10670102,"Agheel Lake"),
		(10670103,"Dragon-Burnt Ruins"),
		(10670104,"Peninsula Minor Erdtree"),
		(10670105,"Castle Morne"),
		(10670106,"Waypoint Ruins"),
		(10670130,"Castleward Tunnel"),
		(10670131,"Secluded Cell"),
		(10670132,"Liftside Chamber"),
		(10670133,"Rampart Tower"),
		(10670134,"Stormveil Cliffside"),
		(10670135,"Stormveil Main Gate"),
		(10670160,"Tombsward Catacombs"),
		(10670161,"Impaler's Catacombs"),
		(10670162,"Stormfoot Catacombs"),
		(10670163,"Deathtouched Catacombs"),
		(10670164,"Murkwater Catacombs"),
		(10670165,""),
		(10670166,""),
		(10670167,""),
		(10670168,""),
		(10670169,""),
		(10670170,"Tombsward Cave"),
		(10670171,"Earthbore Cave"),
		(10670172,"Murkwater Cave"),
		(10670173,"Groveside Cave"),
		(10670174,"Coastal Cave"),
		(10670175,"Highroad Cave"),
		(10670176,""),
		(10670177,""),
		(10670178,""),
		(10670179,""),
		(10670180,"Morne Tunnel"),
		(10670181,"Limgrave Tunnels"),
		(10670200,"Lake Minor Erdtree"),
		(10670201,"Temple Quarter"),
		(10670202,"Village of the Albinaurics"),
		(10670203,"Kingsrealm Ruins"),
		(10670204,"Caria Manor"),
		(10670205,"Uld Minor Erdtree"),
		(10670230,"Main Academy Gate"),
		(10670231,"Schoolhouse Classroom"),
		(10670232,"Church of the Cuckoo"),
		(10670233,"Debate Parlor"),
		(10670234,""),
		(10670235,""),
		(10670236,""),
		(10670237,""),
		(10670238,""),
		(10670239,""),
		(10670240,"Ruin-Strewn Precipice"),
		(10670241,"Ruin-Strewn Precipice Overlook"),
		(10670260,"Black Knife Catacombs"),
		(10670261,"Road's End Catacombs"),
		(10670262,"Cliffbottom Catacombs"),
		(10670263,""),
		(10670264,""),
		(10670265,""),
		(10670266,""),
		(10670267,""),
		(10670268,""),
		(10670269,""),
		(10670270,"Stillwater Cave"),
		(10670271,"Lakeside Crystal Cave"),
		(10670272,"Academy Crystal Cave"),
		(10670273,""),
		(10670274,""),
		(10670275,""),
		(10670276,""),
		(10670277,""),
		(10670278,""),
		(10670279,""),
		(10670280,"Raya Lucaria Crystal Tunnel"),
		(10670300,"Fort Laiedd South"),
		(10670301,"Hermit Village Cliff Road"),
		(10670302,""),
		(10670303,"Road of Iniquity"),
		(10670304,"Craftsman's Shack"),
		(10670305,"Wyndham Ruins"),
		(10670306,"The Shaded Castle"),
		(10670307,"Writheblood Ruins"),
		(10670308,"Dominula, Windmill Village"),
		(10670309,"Outer Wall Minor Erdtree"),
		(10670310,"Outer Wall East"),
		(10670330,"East Capital Rampart"),
		(10670331,"Avenue Balcony"),
		(10670332,"Lower Capital Church"),
		(10670333,"West Capital Rampart"),
		(10670334,"Queen's Bedchamber"),
		(10670335,""),
		(10670336,""),
		(10670337,""),
		(10670338,""),
		(10670339,""),
		(10670340,"Underground Roadside"),
		(10670341,"Forsaken Depths"),
		(10670342,"Leyndell Catacombs"),
		(10670343,""),
		(10670344,""),
		(10670345,""),
		(10670346,""),
		(10670347,""),
		(10670348,""),
		(10670349,""),
		(10670350,""),
		(10670351,"Prison Town Church"),
		(10670352,"Guest Hall"),
		(10670353,"Temple of Eiglay"),
		(10670354,"Audience Pathway"),
		(10670355,""),
		(10670356,""),
		(10670357,""),
		(10670358,""),
		(10670359,""),
		(10670360,"Sainted Hero's Grave"),
		(10670361,"Gelmir Hero's Grave"),
		(10670362,"Auriza Hero's Grave"),
		(10670363,"Unsightly Catacombs"),
		(10670364,"Wyndham Catacombs"),
		(10670365,""),
		(10670366,""),
		(10670367,""),
		(10670368,""),
		(10670369,""),
		(10670370,"Seethewater Cave"),
		(10670371,"Volcano Cave"),
		(10670372,"Perfumer's Grotto"),
		(10670373,"Sage's Cave"),
		(10670374,""),
		(10670375,""),
		(10670376,""),
		(10670377,""),
		(10670378,""),
		(10670379,""),
		(10670380,"Old Altus Tunnel"),
		(10670381,"Altus Tunnel"),
		(10670382,""),
		(10670383,""),
		(10670384,""),
		(10670385,""),
		(10670386,""),
		(10670387,""),
		(10670388,""),
		(10670389,""),
		(10670390,"Sealed Tunnel"),
		(10670391,""),
		(10670392,""),
		(10670393,""),
		(10670394,""),
		(10670395,""),
		(10670396,""),
		(10670397,""),
		(10670398,""),
		(10670399,""),
		(10670400,"Caelid Minor Erdtree"),
		(10670401,"Caelem Ruins"),
		(10670402,"Caelid Highway South"),
		(10670403,"Swamp of Aeonia"),
		(10670404,"Sellia, Town of Sorcery"),
		(10670405,"Greyoll's Dragonbarrow"),
		(10670406,"Redmane Castle"),
		(10670407,"Wailing Dunes South"),
		(10670408,"Dragonbarrow Minor Erdtree"),
		(10670409,"Farum Greatbridge"),
		(10670460,"Minor Erdtree Catacombs"),
		(10670461,"Caelid Catacombs"),
		(10670462,"War-Dead Catacombs"),
		(10670463,""),
		(10670464,""),
		(10670465,""),
		(10670466,""),
		(10670467,""),
		(10670468,""),
		(10670469,""),
		(10670470,"Gaol Cave"),
		(10670471,"Dragonbarrow Cave"),
		(10670472,"Abandoned Cave"),
		(10670473,"Sellia Hideaway"),
		(10670474,""),
		(10670475,""),
		(10670476,""),
		(10670477,""),
		(10670478,""),
		(10670479,""),
		(10670480,"Gael Tunnel"),
		(10670481,"Sellia Crystal Tunnel"),
		(10670482,""),
		(10670483,""),
		(10670484,""),
		(10670485,""),
		(10670486,""),
		(10670487,""),
		(10670488,""),
		(10670489,""),
		(10670490,"Divine Tower of Caelid"),
		(10670491,""),
		(10670492,""),
		(10670493,""),
		(10670494,""),
		(10670495,""),
		(10670496,""),
		(10670497,""),
		(10670498,""),
		(10670499,""),
		(10670500,"Forbidden Lands"),
		(10670501,"Freezing Lake"),
		(10670502,"Foot of the Forge"),
		(10670503,"Mountaintop Minor Erdtree"),
		(10670504,"Ancient Valley Ruins"),
		(10670505,"Castle Sol"),
		(10670506,"Freezing River"),
		(10670507,"Snowfield Minor Erdtree"),
		(10670530,"Haligtree Canopy"),
		(10670531,"Haligtree Town"),
		(10670532,"Haligtree Town Plaza"),
		(10670533,""),
		(10670534,"Haligtree Promenade"),
		(10670535,"Prayer Room"),
		(10670536,"Elphael Inner Wall"),
		(10670537,"Drainage Channel"),
		(10670538,""),
		(10670539,"Haligtree Roots"),
		(10670560,"Giant-Conquering Hero's Grave"),
		(10670561,"Giants' Mountaintop Catacombs"),
		(10670562,"Consecrated Snowfield Catacombs"),
		(10670563,"Hidden Path to the Haligtree"),
		(10670564,""),
		(10670565,""),
		(10670566,""),
		(10670567,""),
		(10670568,""),
		(10670569,""),
		(10670570,"Cave of the Forlorn"),
		(10670571,"Spiritcaller Cave"),
		(10670572,""),
		(10670573,""),
		(10670574,""),
		(10670575,""),
		(10670576,""),
		(10670577,""),
		(10670578,""),
		(10670579,""),
		(10670580,"Yelough Anix Tunnel"),
		(10670610,"Ainsel River Well Depths"),
		(10670611,"Ainsel River Sluice Gate"),
		(10670612,"Ainsel River Downstream"),
		(10670613,"Ainsel River Main"),
		(10670614,"Nokstella, Eternal City"),
		(10670615,"Lake of Rot Shoreside"),
		(10670616,"Grand Cloister"),
		(10670617,""),
		(10670618,""),
		(10670619,""),
		(10670620,"Siofra River Bank"),
		(10670621,"Worshippers' Woods"),
		(10670622,"Mimic Tear"),
		(10670623,"Ancestral Woods"),
		(10670624,"Aqueduct-Facing Cliffs"),
		(10670625,"Night's Sacred Ground"),
		(10670626,"Below the Well"),
		(10670627,""),
		(10670628,""),
		(10670629,""),
		(10670630,"Root-Facing Cliffs"),
		(10670631,"Great Waterfall Crest"),
		(10670632,"Deeproot Depths"),
		(10670633,"Across the Roots"),
		(10670634,"The Nameless Eternal City"),
		(10670650,"Palace Approach Ledge-Road"),
		(10670651,"Dynasty Mausoleum Entrance"),
		(10670652,"Dynasty Mausoleum Midpoint"),
		(10670670,"Siofra River Well Depths"),
		(10670671,"Nokron, Eternal City"),
		(10670730,"Queen's Bedchamber"),
		(10670731,"Erdtree Sanctuary"),
		(10670732,""),
		(10670733,""),
		(10670734,""),
		(10670735,""),
		(10670736,""),
		(10670737,""),
		(10670738,""),
		(10670739,""),
		(10670740,"Crumbling Beast Grave"),
		(10670741,"Crumbling Beast Grave Depths"),
		(10670742,"Tempest-Facing Balcony"),
		(10670743,"Dragon Temple"),
		(10670744,"Dragon Temple Altar"),
		(10670745,"Dragon Temple Lift"),
		(10670746,"Beside the Great Bridge"),
		(10670747,"Dragon Temple Rooftop"),
		(10670748,""),
		(10670749,""),
		(10670750,"Elden Throne"),
		(10670800,"Western Nameless Mausoleum"),
		(10670801,"Lake North of the Greatbridge"),
		(10670802,"Castle Front"),
		(10670803,"Castle-Lord's Chamber"),
		(10670804,"Cerulean Coast"),
		(10670805,"Demi-Human Valley"),
		(10670806,"Southern Nameless Mausoleum"),
		(10670807,"Drake Graveyard"),
		(10670808,"Hidden Grave South"),
		(10670809,"Hidden Grave North"),
		(10670810,"Foot of the Jagged Peak"),
		(10670811,"Jagged Peak Cavern"),
		(10670812,"Jagged Peak Summit"),
		(10670813,"Darklight Catacombs, Lower Level"),
		(10670814,"Manse Hall"),
		(10670815,"Second Floor Chamber"),
		(10670830,"Stone Coffin Fissure"),
		(10670831,"Fissure Depths"),
		(10670832,""),
		(10670833,""),
		(10670834,""),
		(10670835,""),
		(10670836,""),
		(10670837,""),
		(10670838,""),
		(10670839,""),
		(10670840,"Darkroad Sanctuary"),
		(10670841,"Belurat, Tower Settlement"),
		(10670842,"Small Private Altar"),
		(10670843,"Stagefront"),
		(10670844,""),
		(10670845,""),
		(10670846,""),
		(10670847,""),
		(10670848,""),
		(10670849,""),
		(10670850,"Enir-Ilim: Outer Wall"),
		(10670851,"First Rise"),
		(10670852,"Spiral Rise"),
		(10670853,"Cleansing Chamber Anteroom"),
		(10670854,"Divine Gate Front Staircase"),
		(10670855,""),
		(10670856,""),
		(10670857,""),
		(10670858,""),
		(10670859,""),
		(10670860,"Fog Rift Catacombs"),
		(10670861,""),
		(10670862,""),
		(10670863,""),
		(10670864,""),
		(10670865,""),
		(10670866,""),
		(10670867,""),
		(10670868,""),
		(10670869,""),
		(10670870,"Belurat Gaol"),
		(10670871,"Lamenter's Gaol"),
		(10670872,""),
		(10670873,""),
		(10670874,""),
		(10670875,""),
		(10670876,""),
		(10670877,""),
		(10670878,""),
		(10670879,""),
		(10670880,"Dragon's Pit"),
		(10670900,"Moorth Highway"),
		(10670901,"Rauh Base, Bear Woods"),
		(10670902,"Northern Nameless Mausoleum"),
		(10670903,"Eastern Nameless Mausoleum"),
		(10670904,"Shadow Keep, Back Gate"),
		(10670905,""),
		(10670906,"Ancient Ruins of Rauh"),
		(10670907,"Ancient Ruins, Under-Stair"),
		(10670908,"Church of the Bud"),
		(10670909,"Tree-Worship Sanctum"),
		(10670910,"Hinterland"),
		(10670911,"Altus, Bear Woods"),
		(10670912,"Fog Rift Fort"),
		(10670913,"Moorth Highway, South"),
		(10670930,"Miyr Hanging Bell"),
		(10670931,""),
		(10670932,""),
		(10670933,""),
		(10670934,""),
		(10670935,""),
		(10670936,""),
		(10670937,""),
		(10670938,""),
		(10670939,""),
		(10670940,"Shadow Keep Main Gate"),
		(10670941,"Church District Entrance"),
		(10670942,"Sunken Chapel"),
		(10670943,"Tree-Worship Passage"),
		(10670944,""),
		(10670945,"West Rampart"),
		(10670946,""),
		(10670947,""),
		(10670948,""),
		(10670949,""),
		(10670950,"Storehouse, First Floor"),
		(10670951,"Storehouse, Fourth Floor"),
		(10670952,"Storehouse, Seventh Floor"),
		(10670953,"Dark Chamber Entrance"),
		(10670954,""),
		(10670955,"Storehouse, Back Section"),
		(10670956,"Storehouse, Loft"),
		(10670957,""),
		(10670958,""),
		(10670959,""),
		(10670960,"Scorpion River Catacombs"),
		(10670961,"Darklight Catacombs"),
		(10670962,""),
		(10670963,""),
		(10670964,""),
		(10670965,""),
		(10670966,""),
		(10670967,""),
		(10670968,""),
		(10670969,""),
		(10670970,"Bonny Gaol"),
		(10670971,""),
		(10670972,""),
		(10670973,""),
		(10670974,""),
		(10670975,""),
		(10670976,""),
		(10670977,""),
		(10670978,""),
		(10670979,""),
		(10670980,"Rivermouth Cave"),
])});
