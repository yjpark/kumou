use notation_tab::prelude::*;

pub fn main() {
    print_tab(&new_tab());
}

pub fn new_tab() -> Tab {
    tab! {
        "9622d99c-42c6-4f5a-8bc3-d2e862d44181"
        Meta: A Minor 4 _4 64
        Tracks: [
            {chord Chord [
                $duration = _1
                "1" Chord ( 1: 3 5 )
                "2-7" Chord ( 2: 3- 5 7- )
                "3.7" Chord ( 3: 3 5 7- )
                "4" Chord ( 4: 3 5 )
                "5" Chord ( 5: 3 5 )
                "5.7" Chord ( 5: 3 5 7- )
                "6-" Chord ( 6: 3- 5 )
                "6" Chord ( 6: 3 5 )
                "7" Chord ( 7: 3 5 7- )
                $duration = _1_2
                "5,4" Chord [ ( 5: 3 5 ) ( 4: 3 5 ) ] |
            ]}
            {guitar Guitar [
                Fretboard
                $duration = _1
                "Am" Shape ( 0 0 2 2 1 0 )
                "F" Shape ( 0 2 2 1 0 0 ) +1
                "Am;5" Shape ( 0 2 2 0 0 0 ) +5
                "Dm7" Shape ( _ _ 0 2 1 1 )
                "G7" Shape ( 3 2 0 0 0 1 )
                "C" Shape ( 0 3 2 0 1 0 )
                "B7" Shape ( _ 2 1 2 0 2 )
                "E7" Shape ( 0 2 2 1 3 0 )
                "6->1" Pick [ (6 5 4 3 2 1) ] |
                $duration = _1_2
                "G,F"
                "G;1/2" Shape ( 0 2 2 1 0 0 ) +3
                "F;1/2" Shape ( 0 2 2 1 0 0 ) +1 |
                $duration = _1_8
                "intro:1" Pick [ (5 4 3 2 1) 3 2 3 5 1@8 1@7 1@5 ] |
                "intro:2" Pick [ (6 1) 3 2 3 (6 1) 3 2 3 ] |
                "intro:3" Pick [ 4@0 1@10 1@10 1@10 1@10 1@8 1@7 1@8 ] |
                "intro:4" Pick [ 1 3 2 3 (5 4 3 2 1) 3 2 3 ] |
                "5-1" Pick [ 5 4 2 3 1 3, 2, 4, 1, 3, 2, ] |
                "4-1" Pick [ 4 3 2 3 1 3 4, 1, 3, 2, ] |
                "6-1" Pick [ 6 4 2 3 1 3, 2, 4, 1, 3, 2, ] |
                "5-2" Pick [ 5 4 1 2, 3, 4 3 1 2, 3, ] |
                "4-2" Pick [ 4 3 2 3 1 3, 2, 4, 1, 3, 2, ] |
                "6-2" Pick [ 6 4 2 3 1 2 2@0 3 ] |
            ]}
            {lyrics Lyrics [
                $duration = _1_8
                "1:1" Word [ _* "事", "到", "如", "今", @ ""*+ "不", "能", ] |
                "1:2" Word [ "埋" "怨", "你", @ ""** _ "只", "恨", ] |
                "1:3" Word [ "我"*+ "不", "能", "抗" "拒"* "命" ] |
                "1:4" Word [ "运"**+ _ "时" ] |
                "1:5" Word [ "时"*+ "刻" "刻"*+ "沉", "醉", ] |
                "1:6" Word [ "爱" "河", "里", @ ""* _*+ "谁", "知", ] |
                "1:7" Word [ "悲" "剧"+ "早", "已" @ ""* _ "注" ] |
                "1:8" Word [ "定"*** ] |
                "2:1" Word [ _* "闭", "上", "眼", "睛", @ ""*+ "想", "起", ] |
                "2:2" Word [ "你" "的", "情", @ ""** _ "难", "忘", ] |
                "2:3" Word [ "记"*+ "你", "我", "曾" "有"* "的", "约", ] |
                "2:4" Word [ "定"**+ _ "长" ] |
                "2:5" Word [ "夜"*+ "漫" "漫"*+ "默", "默", ] |
                "2:6" Word [ "在" "哭", "泣", @ ""* _*+ "心", "中", ] |
                "2:7" Word [ "无", "限"+ "痛"* "苦"* "呼"+ "唤", ] |
                "2:8" Word [ "你"*** ] |
                "c:1" Word [ "安"** "妮"*+ "我" ] |
                "c:2" Word [ "不" "能" "失"+ "去", "你"** ] |
                "c:3" Word [ "安"** "妮"*+ "我" ] |
                "c:4" Word [ "无" "法" "忘" "记" "你"** ] |
                "c:5" Word [ "安"** "妮"*+ "我", "用", ] |
                "c:6" Word [ "生" "命" "呼"+ "唤", "你"** ] |
                "c:7" Word [ _*+ "永", "远", "地"*+ "爱" ] |
                "c:8" Word [ "你"*** ] |
            ]}
            {vocal Vocal [
                $duration = _1_8
                "v:1" Tone [ _* ^1, ^1, 7, ^1, @ ^1*+ 6, 7, ] |
                "v:2" Tone [ ^1 7, 4, @ 4** _ 7, ^1, ] |
                "v:3" Tone [ ^2*+ ^1, 7, 6 5* 2, 4, ] |
                "v:4" Tone [ 3**+ _ 3, 5, ] |
                "v:5" Tone [ 4*+ 4, 5, 6*+ ^2, ^1, ] |
                "v:6" Tone [ 7 ^1, 6, @ 6* _*+ 6, ^1, ] |
                "v:7_1" Tone [ 7 7+ ^1, 7 @ 7* _ 6 ] |
                "v:8_1" Tone [ 7*** ] |
                "v:7_2" Tone [ 7, 7 ^1, ^1 7 3* ^1+ 5#, ] |
                "v:8_2" Tone [ 6*** ] |
                "c:1" Tone [ ^3** 6*+ ^5 ] |
                "c:2" Tone [ ^5 ^4 ^4+ ^3, ^4** ] |
                "c:3" Tone [ ^2** 5*+ ^4 ] |
                "c:4" Tone [ ^4 ^3 ^3 ^2# ^3** ] |
                "c:5" Tone [ ^3** 6*+ ^5, ^5, ] |
                "c:6" Tone [ ^5 ^4 ^4+ ^3, ^4** ] |
                "c:7" Tone [ _*+ ^3, ^2, 3*+ 7, 6, ] |
                "c:8" Tone [ 6*** ] |
            ]}
        ]
        Sections: [
            intro Intro [
                {
                    chord [ "6-" 1]
                    guitar [ "Am" 1 ; "intro:1" | ]
                }
                {
                    chord [ "5,4" | ]
                    guitar [ "G,F" | ; "intro:2" | ]
                }
                {
                    chord [ "2-7" 1 ]
                    guitar [ "Dm7" 1 ; "intro:3" | ]
                }
                {
                    chord [ "6-" 1]
                    guitar [ "Am;5" 1 ; "intro:4" | ]
                }
            ]
            verse Verse [
                {
                    chord [ "6-" 1]
                    guitar [ "Am" 1 ; "5-1" | ]
                    lyrics [ "1:1" | @ 1 ; "2:1" | @ 2 ]
                    vocal [ "v:1" | ]
                }
                {
                    chord [ "2-7" 1]
                    guitar [ "Dm7" 1 ; "4-1" | ]
                    lyrics [ "1:2" | @ 1 ; "2:2" | @ 2 ]
                    vocal [ "v:2" | ]
                }
                {
                    chord [ "5.7" 1]
                    guitar [ "G7" 1 ; "6-1" | ]
                    lyrics [ "1:3" | @ 1 ; "2:3" | @ 2 ]
                    vocal [ "v:3" | ]
                }
                {
                    chord [ "1" 1]
                    guitar [ "C" 1 ; "5-2" | ]
                    lyrics [ "1:4" | @ 1 ; "2:4" | @ 2 ]
                    vocal [ "v:4" | ]
                }
                {
                    chord [ "4" 1]
                    guitar [ "F" 1 ; "4-2" | ]
                    lyrics [ "1:5" | @ 1 ; "2:5" | @ 2 ]
                    vocal [ "v:5" | ]
                }
                {
                    chord [ "6-" 1]
                    guitar [ "Am" 1 ; "5-1" | ]
                    lyrics [ "1:6" | @ 1 ; "2:6" | @ 2 ]
                    vocal [ "v:6" | ]
                }
                {
                    chord [ "7" 1]
                    guitar [ "B7" 1 ; "5-1" | ]
                    lyrics [ "1:7" | @ 1 ; "2:7" | @ 2 ]
                    vocal [ "v:7_1" | @ 1 ; "v:7_2" | @ 2 ]

                }
                {
                    chord [ "3.7" 1 @ 1 ; "6-" 1 @ 2 ]
                    guitar [ "E7" 1 @ 1 ; "6-2" | @ 1; "Am" 1 @ 2 ; "5-1" | @ 2 ]
                    lyrics [ "1:8" | @ 1 ; "2:8" | @ 2 ]
                    vocal [ "v:8_1" | @ 1 ; "v:8_2" | @ 2 ]
                }
            ]
            chorus Chorus [
                {
                    chord [ "6-" 1 ]
                    guitar [ "Am" 1 ; "5-1" | ]
                    lyrics [ "c:1" | ]
                    vocal [ "c:1" | ]
                }
                {
                    chord [ "2-7" 1]
                    guitar [ "Dm7" 1 ; "4-1" | ]
                    lyrics [ "c:2" | ]
                    vocal [ "c:2" | ]
                }
                {
                    chord [ "5.7" 1]
                    guitar [ "G7" 1 ; "6-1" | ]
                    lyrics [ "c:3" | ]
                    vocal [ "c:3" | ]
                }
                {
                    chord [ "1" 1]
                    guitar [ "C" 1 ; "5-2" | ]
                    lyrics [ "c:4" | ]
                    vocal [ "c:4" | ]
                }
                {
                    chord [ "6-" 1 ]
                    guitar [ "Am" 1 ; "5-1" | ]
                    lyrics [ "c:5" | ]
                    vocal [ "c:5" | ]
                }
                {
                    chord [ "4" 1]
                    guitar [ "F" 1 ; "4-2" | ]
                    lyrics [ "c:6" | ]
                    vocal [ "c:6" | ]
                }
                {
                    chord [ "3.7" 1]
                    guitar [ "E7" 1 ; "6->1" 1 ]
                    lyrics [ "c:7" | ]
                    vocal [ "c:7" | ]
                }
                {
                    chord [ "6-" 1 ]
                    guitar [ "Am" 1 ; "5-1" | ]
                    lyrics [ "c:8" | ]
                    vocal [ "c:8" | ]
                }
            ]
            outro Outro [
                {
                    chord [ "6-" 1 ]
                    guitar [ "Am;5" 1 ; "6->1" 1 ]
                }
            ]
        ]
        Form: intro verse verse chorus chorus outro
    }
}
