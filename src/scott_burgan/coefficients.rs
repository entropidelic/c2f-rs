// Generated from C2F-W `Cell2Fire/FuelModelSpain.cpp` (`initialize_coeff`).

use super::FuelCoefficients;

#[rustfmt::skip]
pub(super) const SCENARIO_1: [FuelCoefficients; 32] = [
    FuelCoefficients { code: 101, spread: [2.575, 0.6665, 0.1096], flame: [-0.516, 0.288, 0.8972] }, // GR1
    FuelCoefficients { code: 102, spread: [0.4218, 0.2915, 0.01212], flame: [-1.307, 0.1322, 1.887] }, // GR2
    FuelCoefficients { code: 103, spread: [0.1638, 0.2095, 0.007809], flame: [-1.572, 0.1068, 2.409] }, // GR3
    FuelCoefficients { code: 104, spread: [0.07138, 0.1417, 0.002243], flame: [-2.391, 0.05973, 3.334] }, // GR4
    FuelCoefficients { code: 105, spread: [0.03548, 0.07829, 0.001524], flame: [-3.075, 0.03373, 4.391] }, // GR5
    FuelCoefficients { code: 106, spread: [0.02469, 0.08049, 0.0008245], flame: [-4.006, 0.03019, 5.463] }, // GR6
    FuelCoefficients { code: 107, spread: [0.02329, 0.09086, 0.0009262], flame: [-3.979, 0.03746, 5.587] }, // GR7
    FuelCoefficients { code: 108, spread: [0.02111, 0.07793, 0.001174], flame: [-3.842, 0.03855, 5.817] }, // GR8
    FuelCoefficients { code: 121, spread: [0.613, 0.2139, 0.0187], flame: [-1.267, 0.0922, 1.845] }, // GS1
    FuelCoefficients { code: 122, spread: [0.195, 0.1243, 0.006478], flame: [-1.939, 0.05084, 2.706] }, // GS2
    FuelCoefficients { code: 123, spread: [0.06515, 0.07833, 0.002831], flame: [-2.646, 0.03365, 3.788] }, // GS3
    FuelCoefficients { code: 124, spread: [0.07867, 0.07788, 0.003431], flame: [-3.508, 0.03246, 5.098] }, // GS4
    FuelCoefficients { code: 142, spread: [0.3505, 0.07768, 0.01498], flame: [-1.959, 0.0322, 2.819] }, // SH2
    FuelCoefficients { code: 143, spread: [1.748, 0.1589, 0.08249], flame: [-0.938, 0.07674, 1.479] }, // SH3
    FuelCoefficients { code: 144, spread: [0.08116, 0.07849, 0.003353], flame: [-2.452, 0.03347, 3.486] }, // SH4
    FuelCoefficients { code: 145, spread: [0.0483, 0.0781, 0.002783], flame: [-2.796, 0.04029, 4.24] }, // SH5
    FuelCoefficients { code: 146, spread: [0.1248, 0.07783, 0.007999], flame: [-2.121, 0.04111, 3.327] }, // SH6
    FuelCoefficients { code: 147, spread: [0.07431, 0.07791, 0.004384], flame: [-2.668, 0.04027, 4.089] }, // SH7
    FuelCoefficients { code: 148, spread: [0.1238, 0.07772, 0.00647], flame: [-2.506, 0.03664, 3.756] }, // SH8
    FuelCoefficients { code: 149, spread: [0.0667, 0.07779, 0.003506], flame: [-3.209, 0.03641, 4.839] }, // SH9
    FuelCoefficients { code: 161, spread: [2.616, 0.1744, 0.1004], flame: [-0.867, 0.08398, 1.326] }, // TU1
    FuelCoefficients { code: 162, spread: [0.6444, 0.1574, 0.02134], flame: [-1.347, 0.0649, 1.958] }, // TU2
    FuelCoefficients { code: 163, spread: [0.08987, 0.07819, 0.003927], flame: [-2.419, 0.03439, 3.493] }, // TU3
    FuelCoefficients { code: 164, spread: [0.1843, 0.07912, 0.005477], flame: [-2.661, 0.0271, 3.579] }, // TU4
    FuelCoefficients { code: 165, spread: [0.3291, 0.07679, 0.02046], flame: [-1.891, 0.03665, 3.027] }, // TU5
    FuelCoefficients { code: 181, spread: [14.96, 0.401, 1.983], flame: [-0.1525, 0.4018, 0.4484] }, // TL1
    FuelCoefficients { code: 182, spread: [11.01, 0.3366, 0.7639], flame: [-0.3704, 0.202, 0.6365] }, // TL2
    FuelCoefficients { code: 183, spread: [10.09, 0.3454, 0.584], flame: [-0.4398, 0.1448, 0.7804] }, // TL3
    FuelCoefficients { code: 185, spread: [2.195, 0.1807, 0.07939], flame: [-0.8838, 0.08234, 1.368] }, // TL5
    FuelCoefficients { code: 186, spread: [0.9183, 0.1224, 0.02969], flame: [-1.306, 0.04578, 1.908] }, // TL6
    FuelCoefficients { code: 188, spread: [0.4789, 0.07735, 0.0195], flame: [-1.68, 0.02617, 2.451] }, // TL8
    FuelCoefficients { code: 189, spread: [0.3229, 0.07716, 0.01346], flame: [-1.916, 0.02961, 2.816] }, // TL9
];

#[rustfmt::skip]
pub(super) const SCENARIO_2: [FuelCoefficients; 32] = [
    FuelCoefficients { code: 101, spread: [4.941, 0.7086, 0.2124], flame: [-0.4535, 0.2682, 0.777] }, // GR1
    FuelCoefficients { code: 102, spread: [0.8112, 0.3045, 0.02528], flame: [-1.036, 0.1385, 1.561] }, // GR2
    FuelCoefficients { code: 103, spread: [0.3609, 0.2318, 0.01738], flame: [-1.273, 0.1135, 1.966] }, // GR3
    FuelCoefficients { code: 104, spread: [0.1486, 0.153, 0.004652], flame: [-1.98, 0.06712, 2.744] }, // GR4
    FuelCoefficients { code: 105, spread: [0.0681, 0.07832, 0.002926], flame: [-2.578, 0.03345, 3.694] }, // GR5
    FuelCoefficients { code: 106, spread: [0.04896, 0.07908, 0.001631], flame: [-3.334, 0.03012, 4.525] }, // GR6
    FuelCoefficients { code: 107, spread: [0.03528, 0.07869, 0.001315], flame: [-3.658, 0.03158, 5.09] }, // GR7
    FuelCoefficients { code: 108, spread: [0.04173, 0.07796, 0.002318], flame: [-3.209, 0.03823, 4.86] }, // GR8
    FuelCoefficients { code: 121, spread: [1.64, 0.2795, 0.05015], flame: [-0.9854, 0.1202, 1.392] }, // GS1
    FuelCoefficients { code: 122, spread: [0.4834, 0.1577, 0.01499], flame: [-1.486, 0.06882, 2.084] }, // GS2
    FuelCoefficients { code: 123, spread: [0.1385, 0.08614, 0.005977], flame: [-2.108, 0.03642, 3.029] }, // GS3
    FuelCoefficients { code: 124, spread: [0.1333, 0.07794, 0.005812], flame: [-2.963, 0.03259, 4.318] }, // GS4
    FuelCoefficients { code: 142, spread: [3.016, 0.1405, 0.117], flame: [-0.8923, 0.06068, 1.351] }, // SH2
    FuelCoefficients { code: 143, spread: [3.263, 0.1853, 0.1567], flame: [-0.7716, 0.08453, 1.244] }, // SH3
    FuelCoefficients { code: 144, spread: [0.2209, 0.1077, 0.00858], flame: [-1.841, 0.04621, 2.634] }, // SH4
    FuelCoefficients { code: 145, spread: [0.07661, 0.07807, 0.004418], flame: [-2.425, 0.03922, 3.681] }, // SH5
    FuelCoefficients { code: 146, spread: [0.1924, 0.07805, 0.01232], flame: [-1.864, 0.04114, 2.908] }, // SH6
    FuelCoefficients { code: 147, spread: [0.118, 0.07792, 0.006965], flame: [-2.312, 0.03944, 3.548] }, // SH7
    FuelCoefficients { code: 148, spread: [0.1863, 0.07774, 0.009734], flame: [-2.218, 0.03714, 3.323] }, // SH8
    FuelCoefficients { code: 149, spread: [0.1014, 0.07776, 0.005332], flame: [-2.845, 0.03678, 4.283] }, // SH9
    FuelCoefficients { code: 161, spread: [5.028, 0.2037, 0.1978], flame: [-0.7292, 0.08708, 1.115] }, // TU1
    FuelCoefficients { code: 162, spread: [1.128, 0.1848, 0.03716], flame: [-1.119, 0.07944, 1.645] }, // TU2
    FuelCoefficients { code: 163, spread: [0.1554, 0.08058, 0.006821], flame: [-2.03, 0.03518, 2.943] }, // TU3
    FuelCoefficients { code: 164, spread: [0.272, 0.07915, 0.008083], flame: [-2.345, 0.02613, 3.187] }, // TU4
    FuelCoefficients { code: 165, spread: [0.4786, 0.0766, 0.02979], flame: [-1.676, 0.03638, 2.685] }, // TU5
    FuelCoefficients { code: 181, spread: [21.49, 0.5351, 3.327], flame: [-0.1561, 0.1462, 0.4506] }, // TL1
    FuelCoefficients { code: 182, spread: [11.87, 0.3804, 1.424], flame: [-0.2634, 0.2267, 0.55] }, // TL2
    FuelCoefficients { code: 183, spread: [16.14, 0.4104, 0.9961], flame: [-0.3704, 0.2021, 0.6365] }, // TL3
    FuelCoefficients { code: 185, spread: [4.008, 0.2228, 0.1393], flame: [-0.7146, 0.09259, 1.158] }, // TL5
    FuelCoefficients { code: 186, spread: [1.783, 0.1537, 0.05478], flame: [-1.056, 0.06279, 1.538] }, // TL6
    FuelCoefficients { code: 188, spread: [0.8048, 0.09159, 0.03249], flame: [-1.364, 0.03338, 2.04] }, // TL8
    FuelCoefficients { code: 189, spread: [0.4263, 0.07714, 0.01775], flame: [-1.758, 0.03012, 2.544] }, // TL9
];

#[rustfmt::skip]
pub(super) const SCENARIO_3: [FuelCoefficients; 32] = [
    FuelCoefficients { code: 101, spread: [10.54, 0.9248, 0.6665], flame: [-0.2781, 0.4427, 0.5494] }, // GR1
    FuelCoefficients { code: 102, spread: [9.046, 0.6935, 0.6243], flame: [-0.2777, 0.4381, 0.5495] }, // GR2
    FuelCoefficients { code: 103, spread: [1.101, 0.2945, 0.05263], flame: [-0.864, 0.1465, 1.422] }, // GR3
    FuelCoefficients { code: 104, spread: [2.828, 0.4191, 0.1083], flame: [-0.6279, 0.2175, 1.004] }, // GR4
    FuelCoefficients { code: 105, spread: [0.2066, 0.09646, 0.008649], flame: [-1.891, 0.04173, 2.707] }, // GR5
    FuelCoefficients { code: 106, spread: [0.1252, 0.07909, 0.004167], flame: [-2.484, 0.0302, 3.375] }, // GR6
    FuelCoefficients { code: 107, spread: [0.2693, 0.1079, 0.009412], flame: [-1.823, 0.04448, 2.545] }, // GR7
    FuelCoefficients { code: 108, spread: [0.09611, 0.07795, 0.005342], flame: [-2.517, 0.03868, 3.788] }, // GR8
    FuelCoefficients { code: 121, spread: [25.52, 0.7635, 1.428], flame: [-0.1525, 0.4017, 0.4484] }, // GS1
    FuelCoefficients { code: 122, spread: [8.922, 0.4899, 0.3328], flame: [-0.4465, 0.1749, 0.7789] }, // GS2
    FuelCoefficients { code: 123, spread: [5.884, 0.3888, 0.2259], flame: [-0.5876, 0.1976, 0.8988] }, // GS3
    FuelCoefficients { code: 124, spread: [0.6756, 0.07785, 0.02949], flame: [-1.518, 0.03362, 2.179] }, // GS4
    FuelCoefficients { code: 142, spread: [10.39, 0.2182, 0.3807], flame: [-0.5648, 0.1069, 0.9037] }, // SH2
    FuelCoefficients { code: 143, spread: [5.59, 0.2107, 0.254], flame: [-0.6817, 0.1007, 1.064] }, // SH3
    FuelCoefficients { code: 144, spread: [5.257, 0.3809, 0.1807], flame: [-0.6294, 0.1919, 0.9523] }, // SH4
    FuelCoefficients { code: 145, spread: [0.1073, 0.0781, 0.00619], flame: [-2.173, 0.04044, 3.293] }, // SH5
    FuelCoefficients { code: 146, spread: [0.2664, 0.07788, 0.01707], flame: [-1.698, 0.03934, 2.652] }, // SH6
    FuelCoefficients { code: 147, spread: [0.1651, 0.07791, 0.009743], flame: [-2.084, 0.03998, 3.193] }, // SH7
    FuelCoefficients { code: 148, spread: [0.2704, 0.07765, 0.0141], flame: [-1.978, 0.03601, 2.975] }, // SH8
    FuelCoefficients { code: 149, spread: [0.1391, 0.07771, 0.007329], flame: [-2.626, 0.03657, 3.945] }, // SH9
    FuelCoefficients { code: 161, spread: [3180.0, 1.235, 3.332], flame: [-0.1562, 0.1467, 0.4505] }, // TU1
    FuelCoefficients { code: 162, spread: [1.631, 0.2024, 0.05519], flame: [-0.9908, 0.08943, 1.472] }, // TU2
    FuelCoefficients { code: 163, spread: [0.3075, 0.09702, 0.01307], flame: [-1.705, 0.04277, 2.452] }, // TU3
    FuelCoefficients { code: 164, spread: [0.4047, 0.0793, 0.01199], flame: [-2.052, 0.02663, 2.765] }, // TU4
    FuelCoefficients { code: 165, spread: [0.6448, 0.07668, 0.03998], flame: [-1.521, 0.03838, 2.417] }, // TU5
    FuelCoefficients { code: 181, spread: [1800.0, 1.178, 4.999], flame: [0.0, 0.0, 0.0] }, // TL1
    FuelCoefficients { code: 182, spread: [38.8, 0.5491, 1.989], flame: [-0.2492, 0.1422, 0.5509] }, // TL2
    FuelCoefficients { code: 183, spread: [11.37, 0.3397, 1.244], flame: [-0.3582, 0.1543, 0.6369] }, // TL3
    FuelCoefficients { code: 185, spread: [5.163, 0.2396, 0.1943], flame: [-0.6817, 0.1007, 1.064] }, // TL5
    FuelCoefficients { code: 186, spread: [2.555, 0.171, 0.07786], flame: [-0.9478, 0.07391, 1.375] }, // TL6
    FuelCoefficients { code: 188, spread: [1.294, 0.1085, 0.04999], flame: [-1.184, 0.04144, 1.768] }, // TL8
    FuelCoefficients { code: 189, spread: [0.534, 0.07749, 0.02216], flame: [-1.624, 0.02893, 2.369] }, // TL9
];

#[rustfmt::skip]
pub(super) const SCENARIO_4: [FuelCoefficients; 32] = [
    FuelCoefficients { code: 101, spread: [0.0, 0.0, -1.0], flame: [0.0, 0.0, 0.0] }, // GR1
    FuelCoefficients { code: 102, spread: [0.0, 0.0, -1.0], flame: [0.0, 0.0, 0.0] }, // GR2
    FuelCoefficients { code: 103, spread: [1.02e+04, 2.51, 10.0], flame: [-0.3984, 0.6339, 0.3173] }, // GR3
    FuelCoefficients { code: 104, spread: [1706.0, 5.835, 4.999], flame: [-0.3984, 0.6331, 0.3173] }, // GR4
    FuelCoefficients { code: 105, spread: [24.51, 0.6818, 1.665], flame: [-0.1528, 0.4083, 0.4483] }, // GR5
    FuelCoefficients { code: 106, spread: [0.0, 0.0, -1.0], flame: [0.0, 0.0, 0.0] }, // GR6
    FuelCoefficients { code: 107, spread: [8.706, 0.397, 0.3983], flame: [-0.5259, 0.19, 0.7787] }, // GR7
    FuelCoefficients { code: 108, spread: [29.63, 0.6425, 1.246], flame: [-0.2634, 0.2268, 0.55] }, // GR8
    FuelCoefficients { code: 121, spread: [1.02e+04, 2.505, 10.0], flame: [-0.3942, 0.6206, 0.3174] }, // GS1
    FuelCoefficients { code: 122, spread: [25.02, 0.7935, 2.0], flame: [-0.153, 0.4129, 0.4483] }, // GS2
    FuelCoefficients { code: 123, spread: [2423.0, 5.898, 3.333], flame: [-0.271, 0.2323, 0.4489] }, // GS3
    FuelCoefficients { code: 124, spread: [3.313, 0.1361, 0.1345], flame: [-0.8735, 0.06301, 1.305] }, // GS4
    FuelCoefficients { code: 142, spread: [23.95, 0.3007, 0.9045], flame: [-0.4438, 0.1155, 0.7152] }, // SH2
    FuelCoefficients { code: 143, spread: [8.823, 0.2347, 0.3672], flame: [-0.602, 0.1209, 0.9543] }, // SH3
    FuelCoefficients { code: 144, spread: [6.334, 0.3903, 0.2486], flame: [-0.5751, 0.1717, 0.8994] }, // SH4
    FuelCoefficients { code: 145, spread: [0.7965, 0.168, 0.04058], flame: [-1.156, 0.08584, 1.756] }, // SH5
    FuelCoefficients { code: 146, spread: [0.3421, 0.07793, 0.02181], flame: [-1.558, 0.0402, 2.452] }, // SH6
    FuelCoefficients { code: 147, spread: [0.9702, 0.1314, 0.05167], flame: [-1.118, 0.0653, 1.746] }, // SH7
    FuelCoefficients { code: 148, spread: [1.44, 0.1175, 0.07096], flame: [-1.079, 0.05247, 1.652] }, // SH8
    FuelCoefficients { code: 149, spread: [0.4281, 0.07778, 0.02246], flame: [-1.631, 0.03661, 2.479] }, // SH9
    FuelCoefficients { code: 161, spread: [1819.0, 1.18, 4.999], flame: [-0.3084, 6.721, 0.3162] }, // TU1
    FuelCoefficients { code: 162, spread: [2.199, 0.2162, 0.07223], flame: [-0.9267, 0.09552, 1.36] }, // TU2
    FuelCoefficients { code: 163, spread: [4.902, 0.2927, 0.1805], flame: [-0.6625, 0.1263, 1.058] }, // TU3
    FuelCoefficients { code: 164, spread: [0.0, 0.0, -1.0], flame: [0.0, 0.0, 0.0] }, // TU4
    FuelCoefficients { code: 165, spread: [0.817, 0.07709, 0.05013], flame: [-1.425, 0.03706, 2.275] }, // TU5
    FuelCoefficients { code: 181, spread: [1804.0, 1.178, 4.999], flame: [0.0, 0.0, 0.0] }, // TL1
    FuelCoefficients { code: 182, spread: [26.13, 0.5009, 2.488], flame: [-0.1525, 0.4025, 0.4484] }, // TL2
    FuelCoefficients { code: 183, spread: [19.31, 0.4546, 1.428], flame: [-0.2634, 0.2268, 0.55] }, // TL3
    FuelCoefficients { code: 185, spread: [5.518, 0.2432, 0.2307], flame: [-0.6166, 0.1085, 1.011] }, // TL5
    FuelCoefficients { code: 186, spread: [2.956, 0.1765, 0.09361], flame: [-0.8538, 0.07918, 1.292] }, // TL6
    FuelCoefficients { code: 188, spread: [1.736, 0.1188, 0.06593], flame: [-1.076, 0.04976, 1.599] }, // TL8
    FuelCoefficients { code: 189, spread: [0.6251, 0.07753, 0.02607], flame: [-1.505, 0.03052, 2.216] }, // TL9
];
