/* Theremin WAVE Table "Phoenix" - 1024 entries full table, amplitude -2048..2048*/

const SINE_TABLE: [i16; 1024] = [
    273, 288, 302, 317, 330, 345, 358, 373, 387, 401, 416, 429, 444, 457, 472, 485, 500, 513, 527,
    540, 555, 568, 581, 596, 609, 623, 636, 650, 663, 676, 690, 703, 716, 730, 743, 756, 770, 783,
    796, 808, 821, 835, 848, 860, 873, 885, 898, 911, 923, 936, 948, 960, 973, 985, 998, 1010,
    1022, 1033, 1046, 1058, 1070, 1081, 1093, 1105, 1116, 1128, 1140, 1151, 1163, 1174, 1185, 1197,
    1207, 1219, 1230, 1241, 1251, 1263, 1273, 1284, 1295, 1305, 1316, 1326, 1336, 1347, 1357, 1368,
    1377, 1387, 1397, 1407, 1417, 1427, 1436, 1446, 1455, 1465, 1474, 1483, 1493, 1502, 1511, 1520,
    1528, 1537, 1546, 1555, 1563, 1572, 1581, 1589, 1598, 1606, 1614, 1623, 1630, 1639, 1646, 1654,
    1662, 1669, 1677, 1684, 1691, 1700, 1707, 1714, 1720, 1727, 1735, 1742, 1749, 1755, 1762, 1768,
    1775, 1781, 1787, 1794, 1800, 1806, 1812, 1818, 1823, 1829, 1835, 1841, 1847, 1851, 1857, 1863,
    1868, 1873, 1878, 1883, 1888, 1893, 1898, 1902, 1907, 1911, 1916, 1921, 1924, 1929, 1933, 1937,
    1941, 1945, 1949, 1953, 1956, 1959, 1963, 1966, 1970, 1973, 1977, 1980, 1982, 1986, 1989, 1992,
    1994, 1997, 2000, 2002, 2004, 2007, 2009, 2011, 2014, 2016, 2018, 2020, 2022, 2023, 2026, 2027,
    2028, 2031, 2032, 2033, 2035, 2036, 2036, 2038, 2039, 2039, 2041, 2041, 2042, 2043, 2043, 2045,
    2045, 2045, 2046, 2045, 2046, 2046, 2047, 2046, 2046, 2046, 2046, 2046, 2045, 2045, 2045, 2044,
    2044, 2043, 2042, 2041, 2041, 2040, 2039, 2038, 2037, 2035, 2034, 2033, 2031, 2030, 2029, 2028,
    2026, 2024, 2023, 2021, 2019, 2018, 2016, 2014, 2012, 2010, 2008, 2006, 2003, 2001, 1999, 1996,
    1994, 1991, 1988, 1986, 1983, 1980, 1977, 1975, 1972, 1969, 1967, 1963, 1960, 1957, 1953, 1951,
    1947, 1944, 1941, 1937, 1934, 1930, 1926, 1923, 1920, 1915, 1912, 1908, 1904, 1900, 1897, 1893,
    1889, 1885, 1880, 1876, 1872, 1867, 1863, 1859, 1855, 1850, 1845, 1841, 1837, 1832, 1828, 1823,
    1818, 1813, 1809, 1803, 1799, 1793, 1789, 1784, 1779, 1773, 1768, 1763, 1758, 1753, 1747, 1742,
    1736, 1731, 1726, 1721, 1715, 1710, 1704, 1698, 1692, 1687, 1681, 1675, 1670, 1664, 1658, 1652,
    1646, 1640, 1634, 1628, 1622, 1616, 1610, 1604, 1597, 1591, 1585, 1579, 1573, 1566, 1559, 1553,
    1546, 1540, 1533, 1527, 1520, 1514, 1507, 1500, 1493, 1486, 1480, 1473, 1466, 1460, 1452, 1445,
    1439, 1431, 1425, 1417, 1411, 1403, 1396, 1389, 1381, 1375, 1367, 1360, 1352, 1345, 1337, 1330,
    1323, 1315, 1308, 1301, 1293, 1285, 1277, 1269, 1262, 1254, 1246, 1239, 1231, 1223, 1215, 1207,
    1199, 1191, 1183, 1176, 1168, 1159, 1151, 1143, 1135, 1127, 1118, 1111, 1102, 1094, 1086, 1077,
    1069, 1061, 1052, 1044, 1035, 1027, 1018, 1010, 1001, 993, 984, 976, 966, 958, 949, 941, 931,
    923, 914, 905, 897, 887, 879, 869, 860, 852, 842, 833, 824, 815, 806, 796, 788, 778, 769, 760,
    750, 741, 732, 722, 713, 703, 694, 685, 675, 666, 656, 647, 637, 627, 618, 608, 599, 588, 579,
    569, 559, 549, 540, 529, 520, 510, 500, 490, 480, 470, 460, 450, 440, 430, 420, 410, 400, 389,
    380, 369, 359, 348, 339, 328, 317, 307, 297, 287, 276, 266, 255, 245, 234, 224, 213, 203, 192,
    182, 171, 160, 150, 139, 128, 118, 107, 97, 85, 75, 64, 54, 42, 32, 21, 10, 0, -11, -22, -33,
    -44, -55, -66, -77, -88, -99, -109, -121, -132, -143, -154, -165, -176, -187, -198, -209, -220,
    -231, -242, -253, -264, -275, -287, -298, -309, -320, -331, -343, -354, -365, -376, -387, -398,
    -410, -421, -432, -443, -455, -465, -477, -488, -499, -511, -521, -532, -544, -555, -567, -578,
    -588, -599, -611, -622, -633, -644, -656, -667, -678, -689, -700, -711, -723, -733, -744, -756,
    -767, -778, -789, -799, -811, -822, -833, -844, -854, -866, -877, -887, -898, -909, -920, -931,
    -941, -952, -963, -974, -985, -996, -1006, -1017, -1028, -1038, -1049, -1060, -1070, -1081,
    -1091, -1101, -1112, -1122, -1133, -1144, -1154, -1163, -1174, -1184, -1195, -1205, -1215,
    -1225, -1235, -1245, -1255, -1265, -1275, -1285, -1295, -1304, -1314, -1324, -1334, -1343,
    -1353, -1363, -1372, -1382, -1392, -1400, -1410, -1419, -1428, -1438, -1447, -1456, -1465,
    -1474, -1483, -1492, -1501, -1509, -1518, -1528, -1536, -1545, -1553, -1562, -1570, -1578,
    -1587, -1595, -1603, -1611, -1619, -1628, -1636, -1643, -1651, -1660, -1667, -1674, -1683,
    -1690, -1697, -1704, -1712, -1719, -1726, -1734, -1741, -1748, -1755, -1762, -1768, -1776,
    -1782, -1788, -1796, -1802, -1808, -1815, -1821, -1826, -1833, -1839, -1844, -1850, -1856,
    -1862, -1867, -1873, -1879, -1884, -1890, -1895, -1899, -1904, -1910, -1915, -1919, -1924,
    -1929, -1933, -1938, -1942, -1946, -1951, -1955, -1959, -1963, -1966, -1970, -1975, -1978,
    -1982, -1985, -1988, -1991, -1994, -1997, -2000, -2003, -2006, -2009, -2012, -2014, -2017,
    -2019, -2020, -2023, -2025, -2027, -2029, -2030, -2032, -2034, -2035, -2036, -2038, -2039,
    -2040, -2041, -2042, -2042, -2043, -2043, -2044, -2044, -2045, -2045, -2046, -2045, -2045,
    -2045, -2044, -2044, -2044, -2043, -2042, -2041, -2041, -2040, -2038, -2037, -2036, -2034,
    -2033, -2031, -2029, -2027, -2026, -2024, -2022, -2020, -2016, -2014, -2012, -2009, -2006,
    -2003, -2001, -1997, -1994, -1991, -1988, -1984, -1981, -1977, -1973, -1969, -1966, -1961,
    -1957, -1952, -1949, -1944, -1939, -1934, -1929, -1924, -1919, -1914, -1909, -1904, -1898,
    -1893, -1888, -1882, -1876, -1871, -1864, -1858, -1852, -1846, -1839, -1833, -1826, -1820,
    -1814, -1806, -1800, -1792, -1785, -1778, -1770, -1764, -1756, -1748, -1740, -1733, -1725,
    -1717, -1709, -1700, -1692, -1684, -1675, -1667, -1658, -1649, -1641, -1632, -1623, -1614,
    -1605, -1595, -1586, -1577, -1568, -1558, -1548, -1539, -1529, -1519, -1509, -1499, -1490,
    -1479, -1468, -1458, -1447, -1438, -1427, -1417, -1405, -1394, -1383, -1373, -1362, -1350,
    -1340, -1329, -1317, -1306, -1294, -1282, -1272, -1260, -1248, -1236, -1224, -1212, -1200,
    -1188, -1176, -1164, -1152, -1140, -1127, -1114, -1102, -1090, -1077, -1065, -1052, -1039,
    -1026, -1014, -1001, -988, -974, -962, -949, -936, -922, -909, -896, -882, -869, -856, -843,
    -829, -815, -801, -788, -775, -760, -747, -733, -719, -706, -692, -678, -664, -649, -636, -622,
    -607, -594, -580, -565, -551, -537, -523, -509, -494, -480, -465, -451, -437, -422, -408, -394,
    -379, -365, -351, -336, -322, -307, -292, -278, -264, -250, -234, -220, -206, -190, -176, -162,
    -147, -133, -118, -103, -89, -74, -60, -44, -30, -16, -1, 13, 28, 42, 57, 71, 86, 100, 115,
    129, 144, 159, 173, 188, 201, 216, 230, 245, 259,
];

pub fn sine1val(pos:usize) -> i16 {
    return SINE_TABLE[pos];
}
