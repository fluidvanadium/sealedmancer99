// async fn old_formats() {
//     [
//         (
//             "./tdm.txt".to_string(),
//             Query::And(vec![set("TDM"), not(type_line("basic"))]),
//         ),
//         (
//             "./dft.txt".to_string(),
//             Query::And(vec![set("DFT"), not(type_line("basic"))]),
//         ),
//         (
//             "./fdn.txt".to_string(),
//             Query::And(vec![set("FDN"), not(type_line("basic"))]),
//         ),
//         (
//             "./apc.txt".to_string(),
//             Query::And(vec![set("APC"), not(type_line("basic"))]),
//         ),
//         (
//             "./dsk.txt".to_string(),
//             Query::And(vec![set("DSK"), not(type_line("basic"))]),
//         ),
//         (
//             "./block_rav.txt".to_string(),
//             Query::And(vec![block("RAV"), not(type_line("basic"))]),
//         ),
//     ];
//     let format = [
//         (
//             "./fin-common.txt".to_string(),
//             Query::And(vec![
//                 set("FIN"),
//                 not(type_line("basic")),
//                 rarity(Rarity::Common),
//             ]),
//         ),
//         (
//             "./fin-uncommon.txt".to_string(),
//             Query::And(vec![
//                 set("FIN"),
//                 not(type_line("basic")),
//                 rarity(Rarity::Uncommon),
//             ]),
//         ),
//         (
//             "./fin-rare.txt".to_string(),
//             Query::And(vec![
//                 set("FIN"),
//                 not(type_line("basic")),
//                 rarity(Rarity::Rare),
//             ]),
//         ),
//         (
//             "./fin-mythic.txt".to_string(),
//             Query::And(vec![
//                 set("FIN"),
//                 not(type_line("basic")),
//                 rarity(Rarity::Mythic),
//             ]),
//         ),
//     ];
//     let format = [(
//         "./commanders.txt".to_string(),
//         Query::And(vec![type_line("legendary"), type_line("creature")]),
//     )];
//     // let formargt = [
//     //     (
//     //         "./for-subset-draft-Bonus.txt".to_string(),
//     //         Query::Or(vec![
//     //             Query::And(vec![rarity(Rarity::Bonus), rebalanced.clone()]),
//     //             Query::And(vec![
//     //                 rarity(Rarity::Bonus),
//     //                 vintage_taste_ban.clone(),
//     //                 not(basics.clone()),
//     //                 not(meld_duds.clone()),
//     //                 not(unfun.clone()),
//     //                 //
//     //                 not(commander_synergy.clone()),
//     //                 // not(draft_involved.clone()),
//     //             ]),
//     //             conspiracy.clone(),
//     //         ]),
//     //     ),
//     //     (
//     //         "./for-subset-draft-Mythic.txt".to_string(),
//     //         Query::Or(vec![
//     //             Query::And(vec![rarity(Rarity::Mythic), rebalanced.clone()]),
//     //             Query::And(vec![
//     //                 rarity(Rarity::Mythic),
//     //                 vintage_taste_ban.clone(),
//     //                 not(basics.clone()),
//     //                 not(meld_duds.clone()),
//     //                 not(unfun.clone()),
//     //                 //
//     //                 not(commander_synergy.clone()),
//     //                 // not(draft_involved.clone()),
//     //             ]),
//     //             conspiracy.clone(),
//     //         ]),
//     //     ),
//     //     (
//     //         "./for-subset-draft-Special.txt".to_string(),
//     //         Query::Or(vec![
//     //             Query::And(vec![rarity(Rarity::Special), rebalanced.clone()]),
//     //             Query::And(vec![
//     //                 rarity(Rarity::Special),
//     //                 vintage_taste_ban.clone(),
//     //                 not(basics.clone()),
//     //                 not(meld_duds.clone()),
//     //                 not(unfun.clone()),
//     //                 //
//     //                 not(commander_synergy.clone()),
//     //                 // not(draft_involved.clone()),
//     //             ]),
//     //             conspiracy.clone(),
//     //         ]),
//     //     ),
//     //     (
//     //         "./for-subset-draft-Rare.txt".to_string(),
//     //         Query::Or(vec![
//     //             Query::And(vec![rarity(Rarity::Rare), rebalanced.clone()]),
//     //             Query::And(vec![
//     //                 rarity(Rarity::Rare),
//     //                 vintage_taste_ban.clone(),
//     //                 not(basics.clone()),
//     //                 not(meld_duds.clone()),
//     //                 not(unfun.clone()),
//     //                 //
//     //                 not(commander_synergy.clone()),
//     //                 // not(draft_involved.clone()),
//     //             ]),
//     //             conspiracy.clone(),
//     //         ]),
//     //     ),
//     //     (
//     //         "./for-subset-draft-Uncommon.txt".to_string(),
//     //         Query::Or(vec![
//     //             Query::And(vec![rarity(Rarity::Uncommon), rebalanced.clone()]),
//     //             Query::And(vec![
//     //                 rarity(Rarity::Uncommon),
//     //                 vintage_taste_ban.clone(),
//     //                 not(basics.clone()),
//     //                 not(meld_duds.clone()),
//     //                 not(unfun.clone()),
//     //                 //
//     //                 not(commander_synergy.clone()),
//     //                 // not(draft_involved.clone()),
//     //             ]),
//     //             conspiracy.clone(),
//     //         ]),
//     //     ),
//     //     (
//     //         "./for-subset-draft-Common.txt".to_string(),
//     //         Query::Or(vec![
//     //             Query::And(vec![rarity(Rarity::Common), rebalanced.clone()]),
//     //             Query::And(vec![
//     //                 rarity(Rarity::Common),
//     //                 vintage_taste_ban.clone(),
//     //                 not(basics.clone()),
//     //                 not(meld_duds.clone()),
//     //                 not(unfun.clone()),
//     //                 //
//     //                 not(commander_synergy.clone()),
//     //                 // not(draft_involved.clone()),
//     //             ]),
//     //             conspiracy.clone(),
//     //         ]),
//     //     ),
//     // ];
// }
