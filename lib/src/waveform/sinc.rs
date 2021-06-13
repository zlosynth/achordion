pub const SINC_FACTOR_1024: [u16; 1024] = [
    16413, 16397, 16418, 16450, 16482, 16515, 16547, 16579, 16611, 16644, 16675, 16707, 16739,
    16770, 16802, 16832, 16863, 16893, 16923, 16953, 16982, 17011, 17039, 17067, 17094, 17120,
    17147, 17172, 17197, 17221, 17244, 17267, 17289, 17310, 17331, 17350, 17369, 17387, 17404,
    17420, 17435, 17450, 17463, 17476, 17487, 17498, 17507, 17515, 17523, 17529, 17535, 17539,
    17542, 17544, 17545, 17545, 17544, 17542, 17538, 17534, 17528, 17522, 17514, 17505, 17495,
    17484, 17472, 17458, 17444, 17429, 17412, 17395, 17376, 17357, 17336, 17315, 17292, 17269,
    17244, 17219, 17193, 17165, 17137, 17109, 17079, 17049, 17017, 16985, 16953, 16919, 16885,
    16851, 16815, 16779, 16743, 16706, 16669, 16631, 16593, 16554, 16515, 16476, 16436, 16396,
    16356, 16316, 16276, 16235, 16195, 16154, 16114, 16073, 16033, 15993, 15953, 15913, 15873,
    15834, 15795, 15756, 15718, 15680, 15643, 15606, 15570, 15534, 15499, 15464, 15431, 15398,
    15365, 15334, 15303, 15273, 15244, 15216, 15189, 15163, 15138, 15114, 15091, 15069, 15049,
    15029, 15011, 14994, 14978, 14963, 14949, 14937, 14926, 14917, 14908, 14901, 14896, 14892,
    14889, 14888, 14888, 14889, 14892, 14896, 14902, 14909, 14917, 14927, 14939, 14952, 14966,
    14982, 14999, 15017, 15037, 15058, 15081, 15105, 15130, 15157, 15185, 15215, 15245, 15277,
    15310, 15344, 15380, 15417, 15454, 15493, 15533, 15574, 15617, 15660, 15704, 15749, 15794,
    15841, 15888, 15937, 15986, 16035, 16086, 16136, 16188, 16240, 16292, 16345, 16398, 16452,
    16506, 16560, 16614, 16669, 16723, 16778, 16832, 16887, 16941, 16996, 17050, 17103, 17157,
    17210, 17263, 17315, 17367, 17418, 17468, 17518, 17567, 17615, 17663, 17710, 17755, 17800,
    17844, 17887, 17928, 17969, 18008, 18046, 18082, 18118, 18152, 18184, 18215, 18245, 18273,
    18299, 18324, 18347, 18369, 18389, 18407, 18423, 18438, 18450, 18461, 18470, 18477, 18482,
    18485, 18487, 18486, 18483, 18478, 18472, 18463, 18452, 18439, 18424, 18407, 18388, 18367,
    18344, 18319, 18291, 18262, 18231, 18198, 18162, 18125, 18086, 18045, 18001, 17956, 17909,
    17861, 17810, 17758, 17703, 17648, 17590, 17531, 17470, 17407, 17343, 17278, 17211, 17143,
    17073, 17002, 16930, 16856, 16781, 16706, 16629, 16551, 16473, 16393, 16313, 16232, 16150,
    16068, 15985, 15902, 15818, 15734, 15650, 15566, 15481, 15397, 15312, 15228, 15144, 15060,
    14977, 14894, 14811, 14729, 14648, 14568, 14488, 14409, 14332, 14255, 14179, 14105, 14032,
    13961, 13891, 13822, 13755, 13690, 13627, 13565, 13505, 13448, 13392, 13339, 13288, 13239,
    13192, 13148, 13107, 13068, 13032, 12998, 12967, 12939, 12914, 12892, 12873, 12857, 12844,
    12835, 12828, 12825, 12825, 12829, 12836, 12846, 12860, 12877, 12898, 12923, 12951, 12983,
    13019, 13058, 13101, 13148, 13199, 13253, 13311, 13373, 13439, 13509, 13582, 13660, 13741,
    13826, 13915, 14007, 14104, 14204, 14308, 14416, 14527, 14642, 14761, 14884, 15010, 15140,
    15273, 15410, 15550, 15693, 15840, 15991, 16144, 16301, 16461, 16624, 16790, 16959, 17131,
    17306, 17483, 17664, 17846, 18032, 18220, 18410, 18603, 18797, 18994, 19193, 19394, 19597,
    19801, 20008, 20216, 20425, 20635, 20847, 21061, 21275, 21490, 21706, 21923, 22141, 22359,
    22577, 22796, 23015, 23234, 23454, 23673, 23892, 24110, 24328, 24546, 24763, 24979, 25194,
    25409, 25622, 25834, 26044, 26253, 26461, 26667, 26871, 27073, 27273, 27472, 27667, 27861,
    28052, 28241, 28427, 28610, 28791, 28968, 29143, 29314, 29482, 29647, 29808, 29966, 30121,
    30272, 30418, 30562, 30701, 30836, 30967, 31095, 31217, 31336, 31450, 31560, 31666, 31767,
    31863, 31955, 32042, 32124, 32201, 32274, 32342, 32405, 32463, 32516, 32564, 32607, 32645,
    32678, 32705, 32728, 32746, 32758, 32765, 32767, 32765, 32756, 32743, 32725, 32701, 32673,
    32639, 32600, 32556, 32508, 32454, 32395, 32331, 32263, 32189, 32111, 32028, 31940, 31848,
    31751, 31649, 31543, 31432, 31317, 31198, 31075, 30947, 30815, 30679, 30539, 30395, 30248,
    30096, 29941, 29783, 29621, 29456, 29287, 29115, 28940, 28762, 28581, 28397, 28211, 28022,
    27830, 27636, 27440, 27242, 27041, 26839, 26634, 26428, 26220, 26011, 25800, 25588, 25375,
    25160, 24945, 24728, 24511, 24294, 24076, 23857, 23638, 23419, 23200, 22980, 22761, 22542,
    22324, 22106, 21889, 21672, 21456, 21241, 21027, 20814, 20602, 20391, 20182, 19975, 19769,
    19565, 19362, 19161, 18963, 18766, 18572, 18380, 18190, 18002, 17817, 17635, 17455, 17278,
    17103, 16932, 16763, 16598, 16435, 16276, 16120, 15967, 15817, 15670, 15527, 15388, 15251,
    15119, 14990, 14864, 14742, 14624, 14509, 14398, 14291, 14188, 14088, 13992, 13900, 13812,
    13728, 13647, 13570, 13497, 13428, 13363, 13302, 13244, 13190, 13140, 13094, 13052, 13013,
    12978, 12946, 12919, 12895, 12874, 12857, 12844, 12834, 12828, 12825, 12825, 12829, 12836,
    12846, 12860, 12876, 12896, 12918, 12944, 12972, 13003, 13037, 13074, 13113, 13155, 13200,
    13246, 13296, 13347, 13401, 13457, 13515, 13575, 13637, 13700, 13766, 13833, 13902, 13972,
    14044, 14117, 14191, 14267, 14344, 14422, 14501, 14580, 14661, 14742, 14824, 14907, 14990,
    15073, 15157, 15241, 15326, 15410, 15495, 15579, 15663, 15748, 15831, 15915, 15998, 16081,
    16163, 16245, 16326, 16406, 16485, 16564, 16641, 16718, 16793, 16868, 16941, 17013, 17084,
    17154, 17222, 17288, 17354, 17417, 17480, 17540, 17599, 17657, 17712, 17766, 17818, 17869,
    17917, 17964, 18008, 18051, 18092, 18131, 18168, 18203, 18236, 18267, 18296, 18323, 18348,
    18370, 18391, 18410, 18427, 18441, 18454, 18464, 18473, 18479, 18484, 18486, 18487, 18485,
    18482, 18476, 18469, 18460, 18448, 18435, 18421, 18404, 18386, 18366, 18344, 18320, 18295,
    18269, 18240, 18210, 18179, 18146, 18112, 18077, 18040, 18002, 17962, 17922, 17880, 17837,
    17793, 17748, 17702, 17656, 17608, 17559, 17510, 17460, 17410, 17358, 17307, 17254, 17202,
    17148, 17095, 17041, 16987, 16933, 16878, 16824, 16769, 16715, 16660, 16606, 16551, 16497,
    16444, 16390, 16337, 16284, 16232, 16180, 16128, 16078, 16027, 15978, 15929, 15881, 15834,
    15787, 15741, 15697, 15653, 15610, 15568, 15527, 15487, 15448, 15411, 15374, 15339, 15305,
    15272, 15240, 15210, 15181, 15153, 15126, 15101, 15077, 15055, 15034, 15014, 14996, 14979,
    14963, 14949, 14937, 14926, 14916, 14908, 14901, 14895, 14891, 14889, 14887, 14888, 14889,
    14892, 14897, 14902, 14910, 14918, 14928, 14939, 14951, 14965, 14980, 14996, 15014, 15032,
    15052, 15073, 15095, 15118, 15142, 15167, 15194, 15221, 15249, 15278, 15308, 15339, 15370,
    15403, 15436, 15470, 15504, 15540, 15575, 15612, 15649, 15686, 15724, 15762, 15801, 15840,
    15880, 15919, 15959, 15999, 16039, 16080, 16120, 16161, 16201, 16242, 16282, 16322, 16363,
    16403, 16442, 16482, 16521, 16560, 16599, 16637, 16675, 16712, 16749, 16785, 16821, 16856,
    16891, 16925, 16958, 16990, 17022, 17053, 17084, 17113, 17142, 17170, 17197, 17223, 17248,
    17272, 17296, 17318, 17339, 17360, 17379, 17398, 17415, 17431, 17446, 17461, 17474, 17486,
    17497, 17506, 17515, 17523, 17529, 17535, 17539, 17542, 17544, 17545, 17545, 17544, 17542,
    17538, 17534, 17528, 17522, 17514, 17506, 17496, 17485, 17474, 17461, 17448, 17433, 17418,
    17401, 17384, 17366, 17347, 17327, 17307, 17286, 17263, 17241, 17217, 17193, 17168, 17142,
    17116, 17090, 17062, 17034, 17006, 16977, 16948, 16919, 16889, 16858, 16828, 16797, 16765,
    16734, 16702, 16670, 16638, 16606, 16574, 16542, 16510, 16477, 16445,
];
pub const SINC_FACTOR_512: [u16; 512] = [
    16443, 16409, 16450, 16515, 16581, 16646, 16710, 16773, 16835, 16896, 16955, 17013, 17069,
    17123, 17174, 17223, 17269, 17312, 17352, 17388, 17421, 17451, 17477, 17498, 17516, 17530,
    17539, 17544, 17545, 17541, 17533, 17521, 17504, 17483, 17457, 17427, 17393, 17355, 17313,
    17267, 17217, 17163, 17106, 17046, 16983, 16916, 16848, 16776, 16703, 16628, 16551, 16472,
    16393, 16313, 16232, 16151, 16070, 15989, 15910, 15831, 15753, 15677, 15603, 15531, 15462,
    15395, 15331, 15271, 15214, 15161, 15112, 15068, 15028, 14992, 14962, 14936, 14916, 14901,
    14892, 14888, 14889, 14897, 14910, 14928, 14953, 14983, 15019, 15060, 15107, 15160, 15217,
    15280, 15347, 15420, 15497, 15578, 15663, 15752, 15845, 15941, 16039, 16141, 16244, 16350,
    16456, 16565, 16673, 16782, 16891, 17000, 17108, 17214, 17319, 17422, 17522, 17619, 17713,
    17804, 17890, 17972, 18049, 18120, 18187, 18247, 18301, 18349, 18390, 18424, 18451, 18471,
    18482, 18486, 18483, 18471, 18451, 18423, 18386, 18342, 18289, 18228, 18159, 18082, 17998,
    17905, 17806, 17699, 17585, 17465, 17338, 17205, 17067, 16923, 16775, 16622, 16466, 16306,
    16143, 15978, 15811, 15643, 15474, 15305, 15137, 14970, 14805, 14642, 14482, 14325, 14173,
    14026, 13885, 13750, 13622, 13501, 13388, 13284, 13189, 13104, 13029, 12965, 12913, 12872,
    12844, 12828, 12826, 12837, 12862, 12901, 12954, 13022, 13105, 13203, 13317, 13445, 13589,
    13748, 13923, 14112, 14317, 14537, 14772, 15021, 15284, 15562, 15853, 16157, 16475, 16804,
    17146, 17498, 17862, 18236, 18619, 19011, 19411, 19819, 20233, 20653, 21079, 21508, 21941,
    22377, 22814, 23253, 23691, 24128, 24564, 24997, 25426, 25851, 26271, 26684, 27090, 27488,
    27877, 28256, 28625, 28983, 29328, 29660, 29979, 30284, 30573, 30847, 31105, 31346, 31569,
    31775, 31962, 32130, 32280, 32410, 32520, 32610, 32680, 32730, 32759, 32767, 32756, 32723,
    32670, 32597, 32503, 32390, 32257, 32104, 31933, 31743, 31534, 31308, 31064, 30804, 30528,
    30236, 29929, 29608, 29273, 28926, 28566, 28196, 27815, 27424, 27025, 26618, 26203, 25783,
    25357, 24927, 24494, 24058, 23620, 23182, 22744, 22306, 21871, 21438, 21009, 20585, 20166,
    19752, 19346, 18947, 18556, 18175, 17803, 17441, 17090, 16750, 16423, 16107, 15805, 15516,
    15241, 14980, 14733, 14500, 14283, 14081, 13893, 13721, 13565, 13423, 13297, 13186, 13091,
    13010, 12944, 12893, 12857, 12834, 12825, 12830, 12847, 12878, 12920, 12975, 13040, 13117,
    13204, 13300, 13406, 13520, 13642, 13771, 13907, 14050, 14198, 14350, 14507, 14668, 14831,
    14997, 15164, 15333, 15502, 15670, 15838, 16005, 16170, 16332, 16491, 16647, 16799, 16947,
    17090, 17227, 17359, 17484, 17604, 17717, 17822, 17921, 18012, 18095, 18171, 18238, 18298,
    18349, 18393, 18428, 18455, 18473, 18484, 18486, 18481, 18468, 18447, 18419, 18384, 18342,
    18293, 18238, 18176, 18109, 18037, 17959, 17876, 17789, 17698, 17604, 17506, 17405, 17302,
    17197, 17090, 16983, 16874, 16765, 16656, 16547, 16439, 16332, 16227, 16124, 16023, 15925,
    15830, 15738, 15649, 15565, 15484, 15408, 15336, 15269, 15207, 15151, 15099, 15053, 15013,
    14978, 14948, 14925, 14907, 14895, 14889, 14888, 14893, 14903, 14919, 14940, 14966, 14998,
    15034, 15075, 15120, 15170, 15223, 15280, 15341, 15406, 15473, 15543, 15615, 15689, 15766,
    15843, 15922, 16002, 16083, 16164, 16245, 16326, 16406, 16485, 16563, 16640, 16715, 16788,
    16859, 16927, 16993, 17056, 17116, 17172, 17225, 17274, 17320, 17361, 17399, 17432, 17462,
    17486, 17507, 17523, 17535, 17542, 17545, 17544, 17538, 17528, 17513, 17495, 17473, 17446,
    17416, 17383, 17345, 17305, 17262, 17215, 17166, 17114, 17060, 17004, 16946, 16886, 16825,
    16763, 16700, 16636, 16571, 16507,
];
pub const SINC_FACTOR_256: [u16; 256] = [
    16502, 16434, 16512, 16644, 16777, 16901, 17017, 17126, 17226, 17315, 17391, 17453, 17500,
    17530, 17544, 17541, 17519, 17481, 17425, 17352, 17263, 17159, 17041, 16911, 16771, 16622,
    16466, 16306, 16145, 15983, 15825, 15671, 15526, 15390, 15267, 15158, 15065, 14990, 14935,
    14901, 14888, 14898, 14930, 14986, 15064, 15164, 15285, 15426, 15585, 15760, 15948, 16149,
    16358, 16573, 16791, 17008, 17222, 17429, 17627, 17810, 17977, 18125, 18251, 18352, 18426,
    18471, 18486, 18469, 18419, 18337, 18222, 18075, 17897, 17690, 17455, 17194, 16912, 16610,
    16293, 15965, 15630, 15292, 14957, 14629, 14314, 14016, 13740, 13492, 13277, 13098, 12962,
    12870, 12828, 12839, 12905, 13029, 13213, 13457, 13762, 14129, 14556, 15042, 15585, 16183,
    16832, 17528, 18266, 19043, 19852, 20687, 21543, 22412, 23288, 24163, 25031, 25885, 26716,
    27519, 28286, 29010, 29686, 30307, 30868, 31364, 31790, 32143, 32419, 32616, 32733, 32767,
    32720, 32590, 32380, 32092, 31727, 31290, 30783, 30212, 29582, 28898, 28167, 27394, 26586,
    25750, 24894, 24024, 23148, 22273, 21406, 20553, 19721, 18917, 18146, 17414, 16725, 16084,
    15495, 14961, 14484, 14066, 13709, 13414, 13179, 13005, 12890, 12833, 12831, 12881, 12980,
    13124, 13308, 13529, 13782, 14061, 14362, 14680, 15010, 15346, 15683, 16017, 16344, 16659,
    16958, 17237, 17494, 17725, 17928, 18101, 18243, 18353, 18430, 18474, 18486, 18466, 18416,
    18338, 18233, 18104, 17952, 17782, 17596, 17397, 17189, 16974, 16756, 16539, 16324, 16117,
    15918, 15731, 15558, 15402, 15265, 15147, 15050, 14976, 14924, 14895, 14888, 14904, 14942,
    15001, 15078, 15174, 15285, 15411, 15548, 15695, 15849, 16009, 16170, 16332, 16491, 16646,
    16793, 16932, 17060, 17176, 17278, 17364, 17435, 17488, 17524, 17542, 17543, 17527, 17493,
    17444, 17380, 17302, 17211, 17110, 16999, 16881, 16758, 16631,
];
pub const SINC_FACTOR_128: [u16; 128] = [
    16622, 16483, 16636, 16893, 17131, 17320, 17456, 17531, 17539, 17476, 17345, 17150, 16901,
    16611, 16295, 15973, 15662, 15382, 15152, 14987, 14900, 14901, 14992, 15173, 15437, 15773,
    16163, 16588, 17023, 17443, 17821, 18133, 18356, 18471, 18464, 18328, 18062, 17672, 17174,
    16588, 15942, 15269, 14608, 13998, 13479, 13091, 12870, 12847, 13045, 13482, 14163, 15084,
    16233, 17584, 19104, 20752, 22478, 24228, 25947, 27577, 29062, 30350, 31397, 32165, 32627,
    32767, 32578, 32068, 31256, 30170, 28848, 27338, 25691, 23964, 22212, 20495, 18863, 17365,
    16042, 14926, 14040, 13396, 12996, 12832, 12886, 13135, 13545, 14081, 14702, 15368, 16039,
    16679, 17254, 17738, 18111, 18358, 18475, 18463, 18331, 18094, 17770, 17384, 16960, 16525,
    16103, 15720, 15393, 15140, 14972, 14894, 14907, 15006, 15181, 15420, 15705, 16019, 16342,
    16656, 16941, 17183, 17369, 17490, 17542, 17524, 17440, 17296, 17102, 16873,
];
pub const SINC_FACTOR_64: [u16; 64] = [
    16865, 16589, 16870, 17288, 17520, 17467, 17137, 16597, 15962, 15378, 14991, 14913, 15191,
    15793, 16606, 17453, 18133, 18458, 18304, 17640, 16554, 15242, 13985, 13100, 12882, 13544,
    15170, 17687, 20861, 24333, 27666, 30414, 32197, 32765, 32035, 30112, 27264, 23884, 20419,
    17302, 14881, 13370, 12824, 13142, 14096, 15386, 16694, 17747, 18359, 18457, 18083, 17371,
    16513, 15712, 15138, 14898, 15014, 15430, 16029, 16663, 17186, 17489, 17519, 17287,
];
pub const SINC_FACTOR_32: [u16; 64] = [
    17271, 16960, 16851, 16985, 17220, 17381, 17344, 17063, 16579, 15993, 15446, 15080, 15003,
    15262, 15827, 16592, 17390, 18029, 18332, 18180, 17548, 16516, 15276, 14096, 13280, 13114,
    13801, 15423, 17908, 21031, 24444, 27720, 30427, 32192, 32766, 32065, 30186, 27385, 24045,
    20601, 17480, 15028, 13463, 12848, 13094, 13986, 15232, 16524, 17589, 18238, 18390, 18076,
    17419, 16602, 15820, 15242, 14976, 15053, 15426, 15986, 16592, 17105, 17417, 17470,
];
pub const SINC_FACTOR_16: [u16; 64] = [
    17018, 17080, 17105, 17160, 17254, 17350, 17395, 17333, 17134, 16802, 16380, 15945, 15588,
    15396, 15427, 15695, 16158, 16723, 17262, 17637, 17732, 17482, 16898, 16070, 15169, 14415,
    14045, 14271, 15231, 16963, 19380, 22279, 25362, 28275, 30665, 32229, 32767, 32210, 30629,
    28230, 25313, 22231, 19330, 16905, 15152, 14155, 13875, 14179, 14864, 15702, 16484, 17052,
    17320, 17279, 16990, 16561, 16114, 15762, 15585, 15610, 15818, 16146, 16511, 16825,
];
pub const SINC_FACTOR_8: [u16; 64] = [
    15943, 16108, 16271, 16452, 16652, 16858, 17042, 17174, 17224, 17178, 17037, 16824, 16578,
    16351, 16191, 16136, 16203, 16379, 16624, 16876, 17062, 17118, 16999, 16700, 16261, 15766,
    15338, 15118, 15246, 15834, 16945, 18577, 20655, 23039, 25537, 27933, 30008, 31579, 32516,
    32760, 32329, 31312, 29849, 28113, 26280, 24506, 22911, 21565, 20488, 19656, 19018, 18505,
    18057, 17625, 17187, 16743, 16316, 15939, 15646, 15463, 15400, 15448, 15580, 15759,
];
pub const SINC_FACTOR_4: [u16; 64] = [
    21030, 20582, 20171, 19808, 19497, 19233, 19001, 18784, 18561, 18318, 18047, 17751, 17445,
    17150, 16892, 16694, 16573, 16529, 16549, 16605, 16659, 16669, 16603, 16444, 16197, 15895,
    15596, 15380, 15333, 15543, 16076, 16972, 18233, 19820, 21657, 23637, 25639, 27539, 29226,
    30617, 31662, 32348, 32698, 32759, 32598, 32282, 31876, 31427, 30964, 30496, 30018, 29513,
    28964, 28356, 27684, 26955, 26185, 25399, 24624, 23885, 23200, 22577, 22015, 21504,
];
pub const SINC_FACTOR_2: [u16; 64] = [
    30619, 30353, 30080, 29808, 29542, 29280, 29018, 28749, 28464, 28155, 27819, 27458, 27077,
    26687, 26304, 25939, 25603, 25300, 25027, 24772, 24519, 24248, 23941, 23588, 23189, 22757,
    22319, 21915, 21591, 21397, 21374, 21555, 21953, 22563, 23359, 24298, 25327, 26385, 27415,
    28368, 29211, 29923, 30503, 30963, 31324, 31610, 31846, 32052, 32237, 32406, 32554, 32670,
    32745, 32767, 32733, 32641, 32498, 32312, 32097, 31865, 31625, 31383, 31140, 30895,
];
pub const SINC_FACTOR_1: [u16; 64] = [
    16384, 17989, 19580, 21140, 22653, 24107, 25486, 26777, 27969, 29049, 30006, 30833, 31520,
    32062, 32453, 32689, 32767, 32689, 32453, 32062, 31520, 30833, 30006, 29049, 27969, 26777,
    25486, 24107, 22653, 21140, 19580, 17989, 16383, 14778, 13187, 11627, 10114, 8660, 7281, 5990,
    4798, 3718, 2761, 1934, 1247, 705, 314, 78, 0, 78, 314, 705, 1247, 1934, 2761, 3718, 4798,
    5990, 7281, 8660, 10114, 11627, 13187, 14778,
];
pub const SINC_FACTORS: [&[u16]; 11] = [
    &SINC_FACTOR_1,
    &SINC_FACTOR_2,
    &SINC_FACTOR_4,
    &SINC_FACTOR_8,
    &SINC_FACTOR_16,
    &SINC_FACTOR_32,
    &SINC_FACTOR_64,
    &SINC_FACTOR_128,
    &SINC_FACTOR_256,
    &SINC_FACTOR_512,
    &SINC_FACTOR_1024,
];
