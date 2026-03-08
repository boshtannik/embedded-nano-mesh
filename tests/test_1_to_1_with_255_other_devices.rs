#[cfg(test)]
mod tests {
    extern crate std;

    use embedded_nano_mesh::{ms, ExactAddressType, LifeTimeType, Node, NodeConfig, NodeString};
    use proto_lab::{NetworkSimulator, WirelessModemFake};
    use std::time::Instant;

    #[test]
    fn test_send_1_to_1_with_255_other_devices() {
        let mut network_simulator = NetworkSimulator::new(1);
        network_simulator.create_ether("1");
        let mut ether = network_simulator.get_ether("1").expect("Can not get ether");

        let mut modem_1 = WirelessModemFake::new("1");
        let mut modem_2 = WirelessModemFake::new("2");
        let mut modem_3 = WirelessModemFake::new("3");
        let mut modem_4 = WirelessModemFake::new("4");
        let mut modem_5 = WirelessModemFake::new("5");
        let mut modem_6 = WirelessModemFake::new("6");
        let mut modem_7 = WirelessModemFake::new("7");
        let mut modem_8 = WirelessModemFake::new("8");
        let mut modem_9 = WirelessModemFake::new("9");
        let mut modem_10 = WirelessModemFake::new("10");
        let mut modem_11 = WirelessModemFake::new("11");
        let mut modem_12 = WirelessModemFake::new("12");
        let mut modem_13 = WirelessModemFake::new("13");
        let mut modem_14 = WirelessModemFake::new("14");
        let mut modem_15 = WirelessModemFake::new("15");
        let mut modem_16 = WirelessModemFake::new("16");
        let mut modem_17 = WirelessModemFake::new("17");
        let mut modem_18 = WirelessModemFake::new("18");
        let mut modem_19 = WirelessModemFake::new("19");
        let mut modem_20 = WirelessModemFake::new("20");
        let mut modem_21 = WirelessModemFake::new("21");
        let mut modem_22 = WirelessModemFake::new("22");
        let mut modem_23 = WirelessModemFake::new("23");
        let mut modem_24 = WirelessModemFake::new("24");
        let mut modem_25 = WirelessModemFake::new("25");
        let mut modem_26 = WirelessModemFake::new("26");
        let mut modem_27 = WirelessModemFake::new("27");
        let mut modem_28 = WirelessModemFake::new("28");
        let mut modem_29 = WirelessModemFake::new("29");
        let mut modem_30 = WirelessModemFake::new("30");
        let mut modem_31 = WirelessModemFake::new("31");
        let mut modem_32 = WirelessModemFake::new("32");
        let mut modem_33 = WirelessModemFake::new("33");
        let mut modem_34 = WirelessModemFake::new("34");
        let mut modem_35 = WirelessModemFake::new("35");
        let mut modem_36 = WirelessModemFake::new("36");
        let mut modem_37 = WirelessModemFake::new("37");
        let mut modem_38 = WirelessModemFake::new("38");
        let mut modem_39 = WirelessModemFake::new("39");
        let mut modem_40 = WirelessModemFake::new("40");
        let mut modem_41 = WirelessModemFake::new("41");
        let mut modem_42 = WirelessModemFake::new("42");
        let mut modem_43 = WirelessModemFake::new("43");
        let mut modem_44 = WirelessModemFake::new("44");
        let mut modem_45 = WirelessModemFake::new("45");
        let mut modem_46 = WirelessModemFake::new("46");
        let mut modem_47 = WirelessModemFake::new("47");
        let mut modem_48 = WirelessModemFake::new("48");
        let mut modem_49 = WirelessModemFake::new("49");
        let mut modem_50 = WirelessModemFake::new("50");
        let mut modem_51 = WirelessModemFake::new("51");
        let mut modem_52 = WirelessModemFake::new("52");
        let mut modem_53 = WirelessModemFake::new("53");
        let mut modem_54 = WirelessModemFake::new("54");
        let mut modem_55 = WirelessModemFake::new("55");
        let mut modem_56 = WirelessModemFake::new("56");
        let mut modem_57 = WirelessModemFake::new("57");
        let mut modem_58 = WirelessModemFake::new("58");
        let mut modem_59 = WirelessModemFake::new("59");
        let mut modem_60 = WirelessModemFake::new("60");
        let mut modem_61 = WirelessModemFake::new("61");
        let mut modem_62 = WirelessModemFake::new("62");
        let mut modem_63 = WirelessModemFake::new("63");
        let mut modem_64 = WirelessModemFake::new("64");
        let mut modem_65 = WirelessModemFake::new("65");
        let mut modem_66 = WirelessModemFake::new("66");
        let mut modem_67 = WirelessModemFake::new("67");
        let mut modem_68 = WirelessModemFake::new("68");
        let mut modem_69 = WirelessModemFake::new("69");
        let mut modem_70 = WirelessModemFake::new("70");
        let mut modem_71 = WirelessModemFake::new("71");
        let mut modem_72 = WirelessModemFake::new("72");
        let mut modem_73 = WirelessModemFake::new("73");
        let mut modem_74 = WirelessModemFake::new("74");
        let mut modem_75 = WirelessModemFake::new("75");
        let mut modem_76 = WirelessModemFake::new("76");
        let mut modem_77 = WirelessModemFake::new("77");
        let mut modem_78 = WirelessModemFake::new("78");
        let mut modem_79 = WirelessModemFake::new("79");
        let mut modem_80 = WirelessModemFake::new("80");
        let mut modem_81 = WirelessModemFake::new("81");
        let mut modem_82 = WirelessModemFake::new("82");
        let mut modem_83 = WirelessModemFake::new("83");
        let mut modem_84 = WirelessModemFake::new("84");
        let mut modem_85 = WirelessModemFake::new("85");
        let mut modem_86 = WirelessModemFake::new("86");
        let mut modem_87 = WirelessModemFake::new("87");
        let mut modem_88 = WirelessModemFake::new("88");
        let mut modem_89 = WirelessModemFake::new("89");
        let mut modem_90 = WirelessModemFake::new("90");
        let mut modem_91 = WirelessModemFake::new("91");
        let mut modem_92 = WirelessModemFake::new("92");
        let mut modem_93 = WirelessModemFake::new("93");
        let mut modem_94 = WirelessModemFake::new("94");
        let mut modem_95 = WirelessModemFake::new("95");
        let mut modem_96 = WirelessModemFake::new("96");
        let mut modem_97 = WirelessModemFake::new("97");
        let mut modem_98 = WirelessModemFake::new("98");
        let mut modem_99 = WirelessModemFake::new("99");
        let mut modem_100 = WirelessModemFake::new("100");
        let mut modem_101 = WirelessModemFake::new("101");
        let mut modem_102 = WirelessModemFake::new("102");
        let mut modem_103 = WirelessModemFake::new("103");
        let mut modem_104 = WirelessModemFake::new("104");
        let mut modem_105 = WirelessModemFake::new("105");
        let mut modem_106 = WirelessModemFake::new("106");
        let mut modem_107 = WirelessModemFake::new("107");
        let mut modem_108 = WirelessModemFake::new("108");
        let mut modem_109 = WirelessModemFake::new("109");
        let mut modem_110 = WirelessModemFake::new("110");
        let mut modem_111 = WirelessModemFake::new("111");
        let mut modem_112 = WirelessModemFake::new("112");
        let mut modem_113 = WirelessModemFake::new("113");
        let mut modem_114 = WirelessModemFake::new("114");
        let mut modem_115 = WirelessModemFake::new("115");
        let mut modem_116 = WirelessModemFake::new("116");
        let mut modem_117 = WirelessModemFake::new("117");
        let mut modem_118 = WirelessModemFake::new("118");
        let mut modem_119 = WirelessModemFake::new("119");
        let mut modem_120 = WirelessModemFake::new("120");
        let mut modem_121 = WirelessModemFake::new("121");
        let mut modem_122 = WirelessModemFake::new("122");
        let mut modem_123 = WirelessModemFake::new("123");
        let mut modem_124 = WirelessModemFake::new("124");
        let mut modem_125 = WirelessModemFake::new("125");
        let mut modem_126 = WirelessModemFake::new("126");
        let mut modem_127 = WirelessModemFake::new("127");
        let mut modem_128 = WirelessModemFake::new("128");
        let mut modem_129 = WirelessModemFake::new("129");
        let mut modem_130 = WirelessModemFake::new("130");
        let mut modem_131 = WirelessModemFake::new("131");
        let mut modem_132 = WirelessModemFake::new("132");
        let mut modem_133 = WirelessModemFake::new("133");
        let mut modem_134 = WirelessModemFake::new("134");
        let mut modem_135 = WirelessModemFake::new("135");
        let mut modem_136 = WirelessModemFake::new("136");
        let mut modem_137 = WirelessModemFake::new("137");
        let mut modem_138 = WirelessModemFake::new("138");
        let mut modem_139 = WirelessModemFake::new("139");
        let mut modem_140 = WirelessModemFake::new("140");
        let mut modem_141 = WirelessModemFake::new("141");
        let mut modem_142 = WirelessModemFake::new("142");
        let mut modem_143 = WirelessModemFake::new("143");
        let mut modem_144 = WirelessModemFake::new("144");
        let mut modem_145 = WirelessModemFake::new("145");
        let mut modem_146 = WirelessModemFake::new("146");
        let mut modem_147 = WirelessModemFake::new("147");
        let mut modem_148 = WirelessModemFake::new("148");
        let mut modem_149 = WirelessModemFake::new("149");
        let mut modem_150 = WirelessModemFake::new("150");
        let mut modem_151 = WirelessModemFake::new("151");
        let mut modem_152 = WirelessModemFake::new("152");
        let mut modem_153 = WirelessModemFake::new("153");
        let mut modem_154 = WirelessModemFake::new("154");
        let mut modem_155 = WirelessModemFake::new("155");
        let mut modem_156 = WirelessModemFake::new("156");
        let mut modem_157 = WirelessModemFake::new("157");
        let mut modem_158 = WirelessModemFake::new("158");
        let mut modem_159 = WirelessModemFake::new("159");
        let mut modem_160 = WirelessModemFake::new("160");
        let mut modem_161 = WirelessModemFake::new("161");
        let mut modem_162 = WirelessModemFake::new("162");
        let mut modem_163 = WirelessModemFake::new("163");
        let mut modem_164 = WirelessModemFake::new("164");
        let mut modem_165 = WirelessModemFake::new("165");
        let mut modem_166 = WirelessModemFake::new("166");
        let mut modem_167 = WirelessModemFake::new("167");
        let mut modem_168 = WirelessModemFake::new("168");
        let mut modem_169 = WirelessModemFake::new("169");
        let mut modem_170 = WirelessModemFake::new("170");
        let mut modem_171 = WirelessModemFake::new("171");
        let mut modem_172 = WirelessModemFake::new("172");
        let mut modem_173 = WirelessModemFake::new("173");
        let mut modem_174 = WirelessModemFake::new("174");
        let mut modem_175 = WirelessModemFake::new("175");
        let mut modem_176 = WirelessModemFake::new("176");
        let mut modem_177 = WirelessModemFake::new("177");
        let mut modem_178 = WirelessModemFake::new("178");
        let mut modem_179 = WirelessModemFake::new("179");
        let mut modem_180 = WirelessModemFake::new("180");
        let mut modem_181 = WirelessModemFake::new("181");
        let mut modem_182 = WirelessModemFake::new("182");
        let mut modem_183 = WirelessModemFake::new("183");
        let mut modem_184 = WirelessModemFake::new("184");
        let mut modem_185 = WirelessModemFake::new("185");
        let mut modem_186 = WirelessModemFake::new("186");
        let mut modem_187 = WirelessModemFake::new("187");
        let mut modem_188 = WirelessModemFake::new("188");
        let mut modem_189 = WirelessModemFake::new("189");
        let mut modem_190 = WirelessModemFake::new("190");
        let mut modem_191 = WirelessModemFake::new("191");
        let mut modem_192 = WirelessModemFake::new("192");
        let mut modem_193 = WirelessModemFake::new("193");
        let mut modem_194 = WirelessModemFake::new("194");
        let mut modem_195 = WirelessModemFake::new("195");
        let mut modem_196 = WirelessModemFake::new("196");
        let mut modem_197 = WirelessModemFake::new("197");
        let mut modem_198 = WirelessModemFake::new("198");
        let mut modem_199 = WirelessModemFake::new("199");
        let mut modem_200 = WirelessModemFake::new("200");
        let mut modem_201 = WirelessModemFake::new("201");
        let mut modem_202 = WirelessModemFake::new("202");
        let mut modem_203 = WirelessModemFake::new("203");
        let mut modem_204 = WirelessModemFake::new("204");
        let mut modem_205 = WirelessModemFake::new("205");
        let mut modem_206 = WirelessModemFake::new("206");
        let mut modem_207 = WirelessModemFake::new("207");
        let mut modem_208 = WirelessModemFake::new("208");
        let mut modem_209 = WirelessModemFake::new("209");
        let mut modem_210 = WirelessModemFake::new("210");
        let mut modem_211 = WirelessModemFake::new("211");
        let mut modem_212 = WirelessModemFake::new("212");
        let mut modem_213 = WirelessModemFake::new("213");
        let mut modem_214 = WirelessModemFake::new("214");
        let mut modem_215 = WirelessModemFake::new("215");
        let mut modem_216 = WirelessModemFake::new("216");
        let mut modem_217 = WirelessModemFake::new("217");
        let mut modem_218 = WirelessModemFake::new("218");
        let mut modem_219 = WirelessModemFake::new("219");
        let mut modem_220 = WirelessModemFake::new("220");
        let mut modem_221 = WirelessModemFake::new("221");
        let mut modem_222 = WirelessModemFake::new("222");
        let mut modem_223 = WirelessModemFake::new("223");
        let mut modem_224 = WirelessModemFake::new("224");
        let mut modem_225 = WirelessModemFake::new("225");
        let mut modem_226 = WirelessModemFake::new("226");
        let mut modem_227 = WirelessModemFake::new("227");
        let mut modem_228 = WirelessModemFake::new("228");
        let mut modem_229 = WirelessModemFake::new("229");
        let mut modem_230 = WirelessModemFake::new("230");
        let mut modem_231 = WirelessModemFake::new("231");
        let mut modem_232 = WirelessModemFake::new("232");
        let mut modem_233 = WirelessModemFake::new("233");
        let mut modem_234 = WirelessModemFake::new("234");
        let mut modem_235 = WirelessModemFake::new("235");
        let mut modem_236 = WirelessModemFake::new("236");
        let mut modem_237 = WirelessModemFake::new("237");
        let mut modem_238 = WirelessModemFake::new("238");
        let mut modem_239 = WirelessModemFake::new("239");
        let mut modem_240 = WirelessModemFake::new("240");
        let mut modem_241 = WirelessModemFake::new("241");
        let mut modem_242 = WirelessModemFake::new("242");
        let mut modem_243 = WirelessModemFake::new("243");
        let mut modem_244 = WirelessModemFake::new("244");
        let mut modem_245 = WirelessModemFake::new("245");
        let mut modem_246 = WirelessModemFake::new("246");
        let mut modem_247 = WirelessModemFake::new("247");
        let mut modem_248 = WirelessModemFake::new("248");
        let mut modem_249 = WirelessModemFake::new("249");
        let mut modem_250 = WirelessModemFake::new("250");
        let mut modem_251 = WirelessModemFake::new("251");
        let mut modem_252 = WirelessModemFake::new("252");
        let mut modem_253 = WirelessModemFake::new("253");
        let mut modem_254 = WirelessModemFake::new("254");
        let mut modem_255 = WirelessModemFake::new("255");

        ether.register_driver(modem_1.clone());
        ether.register_driver(modem_2.clone());
        ether.register_driver(modem_3.clone());
        ether.register_driver(modem_4.clone());
        ether.register_driver(modem_5.clone());
        ether.register_driver(modem_6.clone());
        ether.register_driver(modem_7.clone());
        ether.register_driver(modem_8.clone());
        ether.register_driver(modem_9.clone());
        ether.register_driver(modem_10.clone());
        ether.register_driver(modem_11.clone());
        ether.register_driver(modem_12.clone());
        ether.register_driver(modem_13.clone());
        ether.register_driver(modem_14.clone());
        ether.register_driver(modem_15.clone());
        ether.register_driver(modem_16.clone());
        ether.register_driver(modem_17.clone());
        ether.register_driver(modem_18.clone());
        ether.register_driver(modem_19.clone());
        ether.register_driver(modem_20.clone());
        ether.register_driver(modem_21.clone());
        ether.register_driver(modem_22.clone());
        ether.register_driver(modem_23.clone());
        ether.register_driver(modem_24.clone());
        ether.register_driver(modem_25.clone());
        ether.register_driver(modem_26.clone());
        ether.register_driver(modem_27.clone());
        ether.register_driver(modem_28.clone());
        ether.register_driver(modem_29.clone());
        ether.register_driver(modem_30.clone());
        ether.register_driver(modem_31.clone());
        ether.register_driver(modem_32.clone());
        ether.register_driver(modem_33.clone());
        ether.register_driver(modem_34.clone());
        ether.register_driver(modem_35.clone());
        ether.register_driver(modem_36.clone());
        ether.register_driver(modem_37.clone());
        ether.register_driver(modem_38.clone());
        ether.register_driver(modem_39.clone());
        ether.register_driver(modem_40.clone());
        ether.register_driver(modem_41.clone());
        ether.register_driver(modem_42.clone());
        ether.register_driver(modem_43.clone());
        ether.register_driver(modem_44.clone());
        ether.register_driver(modem_45.clone());
        ether.register_driver(modem_46.clone());
        ether.register_driver(modem_47.clone());
        ether.register_driver(modem_48.clone());
        ether.register_driver(modem_49.clone());
        ether.register_driver(modem_50.clone());
        ether.register_driver(modem_51.clone());
        ether.register_driver(modem_52.clone());
        ether.register_driver(modem_53.clone());
        ether.register_driver(modem_54.clone());
        ether.register_driver(modem_55.clone());
        ether.register_driver(modem_56.clone());
        ether.register_driver(modem_57.clone());
        ether.register_driver(modem_58.clone());
        ether.register_driver(modem_59.clone());
        ether.register_driver(modem_60.clone());
        ether.register_driver(modem_61.clone());
        ether.register_driver(modem_62.clone());
        ether.register_driver(modem_63.clone());
        ether.register_driver(modem_64.clone());
        ether.register_driver(modem_65.clone());
        ether.register_driver(modem_66.clone());
        ether.register_driver(modem_67.clone());
        ether.register_driver(modem_68.clone());
        ether.register_driver(modem_69.clone());
        ether.register_driver(modem_70.clone());
        ether.register_driver(modem_71.clone());
        ether.register_driver(modem_72.clone());
        ether.register_driver(modem_73.clone());
        ether.register_driver(modem_74.clone());
        ether.register_driver(modem_75.clone());
        ether.register_driver(modem_76.clone());
        ether.register_driver(modem_77.clone());
        ether.register_driver(modem_78.clone());
        ether.register_driver(modem_79.clone());
        ether.register_driver(modem_80.clone());
        ether.register_driver(modem_81.clone());
        ether.register_driver(modem_82.clone());
        ether.register_driver(modem_83.clone());
        ether.register_driver(modem_84.clone());
        ether.register_driver(modem_85.clone());
        ether.register_driver(modem_86.clone());
        ether.register_driver(modem_87.clone());
        ether.register_driver(modem_88.clone());
        ether.register_driver(modem_89.clone());
        ether.register_driver(modem_90.clone());
        ether.register_driver(modem_91.clone());
        ether.register_driver(modem_92.clone());
        ether.register_driver(modem_93.clone());
        ether.register_driver(modem_94.clone());
        ether.register_driver(modem_95.clone());
        ether.register_driver(modem_96.clone());
        ether.register_driver(modem_97.clone());
        ether.register_driver(modem_98.clone());
        ether.register_driver(modem_99.clone());
        ether.register_driver(modem_100.clone());
        ether.register_driver(modem_101.clone());
        ether.register_driver(modem_102.clone());
        ether.register_driver(modem_103.clone());
        ether.register_driver(modem_104.clone());
        ether.register_driver(modem_105.clone());
        ether.register_driver(modem_106.clone());
        ether.register_driver(modem_107.clone());
        ether.register_driver(modem_108.clone());
        ether.register_driver(modem_109.clone());
        ether.register_driver(modem_110.clone());
        ether.register_driver(modem_111.clone());
        ether.register_driver(modem_112.clone());
        ether.register_driver(modem_113.clone());
        ether.register_driver(modem_114.clone());
        ether.register_driver(modem_115.clone());
        ether.register_driver(modem_116.clone());
        ether.register_driver(modem_117.clone());
        ether.register_driver(modem_118.clone());
        ether.register_driver(modem_119.clone());
        ether.register_driver(modem_120.clone());
        ether.register_driver(modem_121.clone());
        ether.register_driver(modem_122.clone());
        ether.register_driver(modem_123.clone());
        ether.register_driver(modem_124.clone());
        ether.register_driver(modem_125.clone());
        ether.register_driver(modem_126.clone());
        ether.register_driver(modem_127.clone());
        ether.register_driver(modem_128.clone());
        ether.register_driver(modem_129.clone());
        ether.register_driver(modem_130.clone());
        ether.register_driver(modem_131.clone());
        ether.register_driver(modem_132.clone());
        ether.register_driver(modem_133.clone());
        ether.register_driver(modem_134.clone());
        ether.register_driver(modem_135.clone());
        ether.register_driver(modem_136.clone());
        ether.register_driver(modem_137.clone());
        ether.register_driver(modem_138.clone());
        ether.register_driver(modem_139.clone());
        ether.register_driver(modem_140.clone());
        ether.register_driver(modem_141.clone());
        ether.register_driver(modem_142.clone());
        ether.register_driver(modem_143.clone());
        ether.register_driver(modem_144.clone());
        ether.register_driver(modem_145.clone());
        ether.register_driver(modem_146.clone());
        ether.register_driver(modem_147.clone());
        ether.register_driver(modem_148.clone());
        ether.register_driver(modem_149.clone());
        ether.register_driver(modem_150.clone());
        ether.register_driver(modem_151.clone());
        ether.register_driver(modem_152.clone());
        ether.register_driver(modem_153.clone());
        ether.register_driver(modem_154.clone());
        ether.register_driver(modem_155.clone());
        ether.register_driver(modem_156.clone());
        ether.register_driver(modem_157.clone());
        ether.register_driver(modem_158.clone());
        ether.register_driver(modem_159.clone());
        ether.register_driver(modem_160.clone());
        ether.register_driver(modem_161.clone());
        ether.register_driver(modem_162.clone());
        ether.register_driver(modem_163.clone());
        ether.register_driver(modem_164.clone());
        ether.register_driver(modem_165.clone());
        ether.register_driver(modem_166.clone());
        ether.register_driver(modem_167.clone());
        ether.register_driver(modem_168.clone());
        ether.register_driver(modem_169.clone());
        ether.register_driver(modem_170.clone());
        ether.register_driver(modem_171.clone());
        ether.register_driver(modem_172.clone());
        ether.register_driver(modem_173.clone());
        ether.register_driver(modem_174.clone());
        ether.register_driver(modem_175.clone());
        ether.register_driver(modem_176.clone());
        ether.register_driver(modem_177.clone());
        ether.register_driver(modem_178.clone());
        ether.register_driver(modem_179.clone());
        ether.register_driver(modem_180.clone());
        ether.register_driver(modem_181.clone());
        ether.register_driver(modem_182.clone());
        ether.register_driver(modem_183.clone());
        ether.register_driver(modem_184.clone());
        ether.register_driver(modem_185.clone());
        ether.register_driver(modem_186.clone());
        ether.register_driver(modem_187.clone());
        ether.register_driver(modem_188.clone());
        ether.register_driver(modem_189.clone());
        ether.register_driver(modem_190.clone());
        ether.register_driver(modem_191.clone());
        ether.register_driver(modem_192.clone());
        ether.register_driver(modem_193.clone());
        ether.register_driver(modem_194.clone());
        ether.register_driver(modem_195.clone());
        ether.register_driver(modem_196.clone());
        ether.register_driver(modem_197.clone());
        ether.register_driver(modem_198.clone());
        ether.register_driver(modem_199.clone());
        ether.register_driver(modem_200.clone());
        ether.register_driver(modem_201.clone());
        ether.register_driver(modem_202.clone());
        ether.register_driver(modem_203.clone());
        ether.register_driver(modem_204.clone());
        ether.register_driver(modem_205.clone());
        ether.register_driver(modem_206.clone());
        ether.register_driver(modem_207.clone());
        ether.register_driver(modem_208.clone());
        ether.register_driver(modem_209.clone());
        ether.register_driver(modem_210.clone());
        ether.register_driver(modem_211.clone());
        ether.register_driver(modem_212.clone());
        ether.register_driver(modem_213.clone());
        ether.register_driver(modem_214.clone());
        ether.register_driver(modem_215.clone());
        ether.register_driver(modem_216.clone());
        ether.register_driver(modem_217.clone());
        ether.register_driver(modem_218.clone());
        ether.register_driver(modem_219.clone());
        ether.register_driver(modem_220.clone());
        ether.register_driver(modem_221.clone());
        ether.register_driver(modem_222.clone());
        ether.register_driver(modem_223.clone());
        ether.register_driver(modem_224.clone());
        ether.register_driver(modem_225.clone());
        ether.register_driver(modem_226.clone());
        ether.register_driver(modem_227.clone());
        ether.register_driver(modem_228.clone());
        ether.register_driver(modem_229.clone());
        ether.register_driver(modem_230.clone());
        ether.register_driver(modem_231.clone());
        ether.register_driver(modem_232.clone());
        ether.register_driver(modem_233.clone());
        ether.register_driver(modem_234.clone());
        ether.register_driver(modem_235.clone());
        ether.register_driver(modem_236.clone());
        ether.register_driver(modem_237.clone());
        ether.register_driver(modem_238.clone());
        ether.register_driver(modem_239.clone());
        ether.register_driver(modem_240.clone());
        ether.register_driver(modem_241.clone());
        ether.register_driver(modem_242.clone());
        ether.register_driver(modem_243.clone());
        ether.register_driver(modem_244.clone());
        ether.register_driver(modem_245.clone());
        ether.register_driver(modem_246.clone());
        ether.register_driver(modem_247.clone());
        ether.register_driver(modem_248.clone());
        ether.register_driver(modem_249.clone());
        ether.register_driver(modem_250.clone());
        ether.register_driver(modem_251.clone());
        ether.register_driver(modem_252.clone());
        ether.register_driver(modem_253.clone());
        ether.register_driver(modem_254.clone());
        ether.register_driver(modem_255.clone());

        let mut node_1 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(1).unwrap(),
            listen_period: 10 as ms,
        });

        let mut node_2 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(2).unwrap(),
            listen_period: 11 as ms,
        });

        let mut node_3 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(3).unwrap(),
            listen_period: 12 as ms,
        });

        let mut node_4 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(4).unwrap(),
            listen_period: 13 as ms,
        });

        let mut node_5 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(5).unwrap(),
            listen_period: 14 as ms,
        });

        let mut node_6 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(6).unwrap(),
            listen_period: 15 as ms,
        });

        let mut node_7 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(7).unwrap(),
            listen_period: 16 as ms,
        });

        let mut node_8 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(8).unwrap(),
            listen_period: 17 as ms,
        });

        let mut node_9 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(9).unwrap(),
            listen_period: 18 as ms,
        });

        let mut node_10 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(10).unwrap(),
            listen_period: 19 as ms,
        });

        let mut node_11 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(11).unwrap(),
            listen_period: 20 as ms,
        });

        let mut node_12 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(12).unwrap(),
            listen_period: 21 as ms,
        });

        let mut node_13 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(13).unwrap(),
            listen_period: 22 as ms,
        });

        let mut node_14 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(14).unwrap(),
            listen_period: 23 as ms,
        });

        let mut node_15 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(15).unwrap(),
            listen_period: 24 as ms,
        });

        let mut node_16 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(16).unwrap(),
            listen_period: 25 as ms,
        });

        let mut node_17 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(17).unwrap(),
            listen_period: 26 as ms,
        });

        let mut node_18 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(18).unwrap(),
            listen_period: 27 as ms,
        });

        let mut node_19 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(19).unwrap(),
            listen_period: 28 as ms,
        });

        let mut node_20 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(20).unwrap(),
            listen_period: 29 as ms,
        });

        let mut node_21 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(21).unwrap(),
            listen_period: 30 as ms,
        });

        let mut node_22 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(22).unwrap(),
            listen_period: 31 as ms,
        });

        let mut node_23 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(23).unwrap(),
            listen_period: 32 as ms,
        });

        let mut node_24 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(24).unwrap(),
            listen_period: 33 as ms,
        });

        let mut node_25 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(25).unwrap(),
            listen_period: 34 as ms,
        });

        let mut node_26 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(26).unwrap(),
            listen_period: 35 as ms,
        });

        let mut node_27 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(27).unwrap(),
            listen_period: 36 as ms,
        });

        let mut node_28 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(28).unwrap(),
            listen_period: 37 as ms,
        });

        let mut node_29 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(29).unwrap(),
            listen_period: 38 as ms,
        });

        let mut node_30 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(30).unwrap(),
            listen_period: 39 as ms,
        });

        let mut node_31 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(31).unwrap(),
            listen_period: 40 as ms,
        });

        let mut node_32 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(32).unwrap(),
            listen_period: 41 as ms,
        });

        let mut node_33 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(33).unwrap(),
            listen_period: 42 as ms,
        });

        let mut node_34 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(34).unwrap(),
            listen_period: 43 as ms,
        });

        let mut node_35 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(35).unwrap(),
            listen_period: 44 as ms,
        });

        let mut node_36 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(36).unwrap(),
            listen_period: 45 as ms,
        });

        let mut node_37 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(37).unwrap(),
            listen_period: 46 as ms,
        });

        let mut node_38 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(38).unwrap(),
            listen_period: 47 as ms,
        });

        let mut node_39 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(39).unwrap(),
            listen_period: 48 as ms,
        });

        let mut node_40 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(40).unwrap(),
            listen_period: 49 as ms,
        });

        let mut node_41 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(41).unwrap(),
            listen_period: 50 as ms,
        });

        let mut node_42 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(42).unwrap(),
            listen_period: 51 as ms,
        });

        let mut node_43 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(43).unwrap(),
            listen_period: 52 as ms,
        });

        let mut node_44 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(44).unwrap(),
            listen_period: 53 as ms,
        });

        let mut node_45 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(45).unwrap(),
            listen_period: 54 as ms,
        });

        let mut node_46 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(46).unwrap(),
            listen_period: 55 as ms,
        });

        let mut node_47 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(47).unwrap(),
            listen_period: 56 as ms,
        });

        let mut node_48 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(48).unwrap(),
            listen_period: 57 as ms,
        });

        let mut node_49 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(49).unwrap(),
            listen_period: 58 as ms,
        });

        let mut node_50 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(50).unwrap(),
            listen_period: 59 as ms,
        });

        let mut node_51 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(51).unwrap(),
            listen_period: 60 as ms,
        });

        let mut node_52 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(52).unwrap(),
            listen_period: 61 as ms,
        });

        let mut node_53 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(53).unwrap(),
            listen_period: 62 as ms,
        });

        let mut node_54 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(54).unwrap(),
            listen_period: 63 as ms,
        });

        let mut node_55 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(55).unwrap(),
            listen_period: 64 as ms,
        });

        let mut node_56 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(56).unwrap(),
            listen_period: 65 as ms,
        });

        let mut node_57 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(57).unwrap(),
            listen_period: 66 as ms,
        });

        let mut node_58 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(58).unwrap(),
            listen_period: 67 as ms,
        });

        let mut node_59 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(59).unwrap(),
            listen_period: 68 as ms,
        });

        let mut node_60 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(60).unwrap(),
            listen_period: 69 as ms,
        });

        let mut node_61 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(61).unwrap(),
            listen_period: 70 as ms,
        });

        let mut node_62 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(62).unwrap(),
            listen_period: 71 as ms,
        });

        let mut node_63 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(63).unwrap(),
            listen_period: 72 as ms,
        });

        let mut node_64 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(64).unwrap(),
            listen_period: 73 as ms,
        });

        let mut node_65 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(65).unwrap(),
            listen_period: 74 as ms,
        });

        let mut node_66 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(66).unwrap(),
            listen_period: 75 as ms,
        });

        let mut node_67 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(67).unwrap(),
            listen_period: 76 as ms,
        });

        let mut node_68 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(68).unwrap(),
            listen_period: 77 as ms,
        });

        let mut node_69 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(69).unwrap(),
            listen_period: 78 as ms,
        });

        let mut node_70 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(70).unwrap(),
            listen_period: 79 as ms,
        });

        let mut node_71 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(71).unwrap(),
            listen_period: 80 as ms,
        });

        let mut node_72 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(72).unwrap(),
            listen_period: 81 as ms,
        });

        let mut node_73 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(73).unwrap(),
            listen_period: 82 as ms,
        });

        let mut node_74 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(74).unwrap(),
            listen_period: 83 as ms,
        });

        let mut node_75 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(75).unwrap(),
            listen_period: 84 as ms,
        });

        let mut node_76 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(76).unwrap(),
            listen_period: 85 as ms,
        });

        let mut node_77 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(77).unwrap(),
            listen_period: 86 as ms,
        });

        let mut node_78 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(78).unwrap(),
            listen_period: 87 as ms,
        });

        let mut node_79 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(79).unwrap(),
            listen_period: 88 as ms,
        });

        let mut node_80 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(80).unwrap(),
            listen_period: 89 as ms,
        });

        let mut node_81 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(81).unwrap(),
            listen_period: 90 as ms,
        });

        let mut node_82 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(82).unwrap(),
            listen_period: 91 as ms,
        });

        let mut node_83 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(83).unwrap(),
            listen_period: 92 as ms,
        });

        let mut node_84 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(84).unwrap(),
            listen_period: 93 as ms,
        });

        let mut node_85 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(85).unwrap(),
            listen_period: 94 as ms,
        });

        let mut node_86 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(86).unwrap(),
            listen_period: 95 as ms,
        });

        let mut node_87 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(87).unwrap(),
            listen_period: 96 as ms,
        });

        let mut node_88 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(88).unwrap(),
            listen_period: 97 as ms,
        });

        let mut node_89 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(89).unwrap(),
            listen_period: 98 as ms,
        });

        let mut node_90 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(90).unwrap(),
            listen_period: 99 as ms,
        });

        let mut node_91 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(91).unwrap(),
            listen_period: 100 as ms,
        });

        let mut node_92 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(92).unwrap(),
            listen_period: 101 as ms,
        });

        let mut node_93 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(93).unwrap(),
            listen_period: 102 as ms,
        });

        let mut node_94 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(94).unwrap(),
            listen_period: 103 as ms,
        });

        let mut node_95 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(95).unwrap(),
            listen_period: 104 as ms,
        });

        let mut node_96 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(96).unwrap(),
            listen_period: 105 as ms,
        });

        let mut node_97 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(97).unwrap(),
            listen_period: 106 as ms,
        });

        let mut node_98 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(98).unwrap(),
            listen_period: 107 as ms,
        });

        let mut node_99 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(99).unwrap(),
            listen_period: 108 as ms,
        });

        let mut node_100 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(100).unwrap(),
            listen_period: 109 as ms,
        });

        let mut node_101 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(101).unwrap(),
            listen_period: 110 as ms,
        });

        let mut node_102 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(102).unwrap(),
            listen_period: 111 as ms,
        });

        let mut node_103 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(103).unwrap(),
            listen_period: 112 as ms,
        });

        let mut node_104 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(104).unwrap(),
            listen_period: 113 as ms,
        });

        let mut node_105 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(105).unwrap(),
            listen_period: 114 as ms,
        });

        let mut node_106 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(106).unwrap(),
            listen_period: 115 as ms,
        });

        let mut node_107 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(107).unwrap(),
            listen_period: 116 as ms,
        });

        let mut node_108 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(108).unwrap(),
            listen_period: 117 as ms,
        });

        let mut node_109 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(109).unwrap(),
            listen_period: 118 as ms,
        });

        let mut node_110 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(110).unwrap(),
            listen_period: 119 as ms,
        });

        let mut node_111 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(111).unwrap(),
            listen_period: 120 as ms,
        });

        let mut node_112 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(112).unwrap(),
            listen_period: 121 as ms,
        });

        let mut node_113 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(113).unwrap(),
            listen_period: 122 as ms,
        });

        let mut node_114 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(114).unwrap(),
            listen_period: 123 as ms,
        });

        let mut node_115 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(115).unwrap(),
            listen_period: 124 as ms,
        });

        let mut node_116 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(116).unwrap(),
            listen_period: 125 as ms,
        });

        let mut node_117 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(117).unwrap(),
            listen_period: 126 as ms,
        });

        let mut node_118 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(118).unwrap(),
            listen_period: 127 as ms,
        });

        let mut node_119 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(119).unwrap(),
            listen_period: 128 as ms,
        });

        let mut node_120 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(120).unwrap(),
            listen_period: 129 as ms,
        });

        let mut node_121 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(121).unwrap(),
            listen_period: 130 as ms,
        });

        let mut node_122 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(122).unwrap(),
            listen_period: 131 as ms,
        });

        let mut node_123 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(123).unwrap(),
            listen_period: 132 as ms,
        });

        let mut node_124 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(124).unwrap(),
            listen_period: 133 as ms,
        });

        let mut node_125 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(125).unwrap(),
            listen_period: 134 as ms,
        });

        let mut node_126 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(126).unwrap(),
            listen_period: 135 as ms,
        });

        let mut node_127 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(127).unwrap(),
            listen_period: 136 as ms,
        });

        let mut node_128 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(128).unwrap(),
            listen_period: 137 as ms,
        });

        let mut node_129 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(129).unwrap(),
            listen_period: 138 as ms,
        });

        let mut node_130 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(130).unwrap(),
            listen_period: 139 as ms,
        });

        let mut node_131 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(131).unwrap(),
            listen_period: 140 as ms,
        });

        let mut node_132 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(132).unwrap(),
            listen_period: 141 as ms,
        });

        let mut node_133 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(133).unwrap(),
            listen_period: 142 as ms,
        });

        let mut node_134 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(134).unwrap(),
            listen_period: 143 as ms,
        });

        let mut node_135 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(135).unwrap(),
            listen_period: 144 as ms,
        });

        let mut node_136 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(136).unwrap(),
            listen_period: 145 as ms,
        });

        let mut node_137 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(137).unwrap(),
            listen_period: 146 as ms,
        });

        let mut node_138 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(138).unwrap(),
            listen_period: 147 as ms,
        });

        let mut node_139 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(139).unwrap(),
            listen_period: 148 as ms,
        });

        let mut node_140 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(140).unwrap(),
            listen_period: 149 as ms,
        });

        let mut node_141 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(141).unwrap(),
            listen_period: 150 as ms,
        });

        let mut node_142 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(142).unwrap(),
            listen_period: 151 as ms,
        });

        let mut node_143 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(143).unwrap(),
            listen_period: 152 as ms,
        });

        let mut node_144 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(144).unwrap(),
            listen_period: 153 as ms,
        });

        let mut node_145 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(145).unwrap(),
            listen_period: 154 as ms,
        });

        let mut node_146 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(146).unwrap(),
            listen_period: 155 as ms,
        });

        let mut node_147 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(147).unwrap(),
            listen_period: 156 as ms,
        });

        let mut node_148 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(148).unwrap(),
            listen_period: 157 as ms,
        });

        let mut node_149 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(149).unwrap(),
            listen_period: 158 as ms,
        });

        let mut node_150 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(150).unwrap(),
            listen_period: 159 as ms,
        });

        let mut node_151 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(151).unwrap(),
            listen_period: 160 as ms,
        });

        let mut node_152 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(152).unwrap(),
            listen_period: 161 as ms,
        });

        let mut node_153 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(153).unwrap(),
            listen_period: 162 as ms,
        });

        let mut node_154 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(154).unwrap(),
            listen_period: 163 as ms,
        });

        let mut node_155 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(155).unwrap(),
            listen_period: 164 as ms,
        });

        let mut node_156 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(156).unwrap(),
            listen_period: 165 as ms,
        });

        let mut node_157 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(157).unwrap(),
            listen_period: 166 as ms,
        });

        let mut node_158 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(158).unwrap(),
            listen_period: 167 as ms,
        });

        let mut node_159 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(159).unwrap(),
            listen_period: 168 as ms,
        });

        let mut node_160 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(160).unwrap(),
            listen_period: 169 as ms,
        });

        let mut node_161 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(161).unwrap(),
            listen_period: 170 as ms,
        });

        let mut node_162 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(162).unwrap(),
            listen_period: 171 as ms,
        });

        let mut node_163 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(163).unwrap(),
            listen_period: 172 as ms,
        });

        let mut node_164 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(164).unwrap(),
            listen_period: 173 as ms,
        });

        let mut node_165 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(165).unwrap(),
            listen_period: 174 as ms,
        });

        let mut node_166 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(166).unwrap(),
            listen_period: 175 as ms,
        });

        let mut node_167 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(167).unwrap(),
            listen_period: 176 as ms,
        });

        let mut node_168 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(168).unwrap(),
            listen_period: 177 as ms,
        });

        let mut node_169 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(169).unwrap(),
            listen_period: 178 as ms,
        });

        let mut node_170 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(170).unwrap(),
            listen_period: 179 as ms,
        });

        let mut node_171 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(171).unwrap(),
            listen_period: 180 as ms,
        });

        let mut node_172 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(172).unwrap(),
            listen_period: 181 as ms,
        });

        let mut node_173 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(173).unwrap(),
            listen_period: 182 as ms,
        });

        let mut node_174 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(174).unwrap(),
            listen_period: 183 as ms,
        });

        let mut node_175 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(175).unwrap(),
            listen_period: 184 as ms,
        });

        let mut node_176 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(176).unwrap(),
            listen_period: 185 as ms,
        });

        let mut node_177 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(177).unwrap(),
            listen_period: 186 as ms,
        });

        let mut node_178 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(178).unwrap(),
            listen_period: 187 as ms,
        });

        let mut node_179 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(179).unwrap(),
            listen_period: 188 as ms,
        });

        let mut node_180 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(180).unwrap(),
            listen_period: 189 as ms,
        });

        let mut node_181 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(181).unwrap(),
            listen_period: 190 as ms,
        });

        let mut node_182 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(182).unwrap(),
            listen_period: 191 as ms,
        });

        let mut node_183 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(183).unwrap(),
            listen_period: 192 as ms,
        });

        let mut node_184 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(184).unwrap(),
            listen_period: 193 as ms,
        });

        let mut node_185 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(185).unwrap(),
            listen_period: 194 as ms,
        });

        let mut node_186 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(186).unwrap(),
            listen_period: 195 as ms,
        });

        let mut node_187 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(187).unwrap(),
            listen_period: 196 as ms,
        });

        let mut node_188 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(188).unwrap(),
            listen_period: 197 as ms,
        });

        let mut node_189 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(189).unwrap(),
            listen_period: 198 as ms,
        });

        let mut node_190 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(190).unwrap(),
            listen_period: 199 as ms,
        });

        let mut node_191 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(191).unwrap(),
            listen_period: 200 as ms,
        });

        let mut node_192 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(192).unwrap(),
            listen_period: 201 as ms,
        });

        let mut node_193 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(193).unwrap(),
            listen_period: 202 as ms,
        });

        let mut node_194 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(194).unwrap(),
            listen_period: 203 as ms,
        });

        let mut node_195 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(195).unwrap(),
            listen_period: 204 as ms,
        });

        let mut node_196 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(196).unwrap(),
            listen_period: 205 as ms,
        });

        let mut node_197 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(197).unwrap(),
            listen_period: 206 as ms,
        });

        let mut node_198 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(198).unwrap(),
            listen_period: 207 as ms,
        });

        let mut node_199 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(199).unwrap(),
            listen_period: 208 as ms,
        });

        let mut node_200 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(200).unwrap(),
            listen_period: 209 as ms,
        });

        let mut node_201 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(201).unwrap(),
            listen_period: 210 as ms,
        });

        let mut node_202 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(202).unwrap(),
            listen_period: 211 as ms,
        });

        let mut node_203 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(203).unwrap(),
            listen_period: 212 as ms,
        });

        let mut node_204 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(204).unwrap(),
            listen_period: 213 as ms,
        });

        let mut node_205 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(205).unwrap(),
            listen_period: 214 as ms,
        });

        let mut node_206 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(206).unwrap(),
            listen_period: 215 as ms,
        });

        let mut node_207 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(207).unwrap(),
            listen_period: 216 as ms,
        });

        let mut node_208 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(208).unwrap(),
            listen_period: 217 as ms,
        });

        let mut node_209 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(209).unwrap(),
            listen_period: 218 as ms,
        });

        let mut node_210 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(210).unwrap(),
            listen_period: 219 as ms,
        });

        let mut node_211 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(211).unwrap(),
            listen_period: 220 as ms,
        });

        let mut node_212 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(212).unwrap(),
            listen_period: 221 as ms,
        });

        let mut node_213 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(213).unwrap(),
            listen_period: 222 as ms,
        });

        let mut node_214 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(214).unwrap(),
            listen_period: 223 as ms,
        });

        let mut node_215 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(215).unwrap(),
            listen_period: 224 as ms,
        });

        let mut node_216 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(216).unwrap(),
            listen_period: 225 as ms,
        });

        let mut node_217 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(217).unwrap(),
            listen_period: 226 as ms,
        });

        let mut node_218 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(218).unwrap(),
            listen_period: 227 as ms,
        });

        let mut node_219 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(219).unwrap(),
            listen_period: 228 as ms,
        });

        let mut node_220 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(220).unwrap(),
            listen_period: 229 as ms,
        });

        let mut node_221 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(221).unwrap(),
            listen_period: 230 as ms,
        });

        let mut node_222 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(222).unwrap(),
            listen_period: 231 as ms,
        });

        let mut node_223 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(223).unwrap(),
            listen_period: 232 as ms,
        });

        let mut node_224 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(224).unwrap(),
            listen_period: 233 as ms,
        });

        let mut node_225 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(225).unwrap(),
            listen_period: 234 as ms,
        });

        let mut node_226 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(226).unwrap(),
            listen_period: 235 as ms,
        });

        let mut node_227 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(227).unwrap(),
            listen_period: 236 as ms,
        });

        let mut node_228 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(228).unwrap(),
            listen_period: 237 as ms,
        });

        let mut node_229 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(229).unwrap(),
            listen_period: 238 as ms,
        });

        let mut node_230 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(230).unwrap(),
            listen_period: 239 as ms,
        });

        let mut node_231 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(231).unwrap(),
            listen_period: 240 as ms,
        });

        let mut node_232 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(232).unwrap(),
            listen_period: 241 as ms,
        });

        let mut node_233 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(233).unwrap(),
            listen_period: 242 as ms,
        });

        let mut node_234 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(234).unwrap(),
            listen_period: 243 as ms,
        });

        let mut node_235 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(235).unwrap(),
            listen_period: 244 as ms,
        });

        let mut node_236 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(236).unwrap(),
            listen_period: 245 as ms,
        });

        let mut node_237 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(237).unwrap(),
            listen_period: 246 as ms,
        });

        let mut node_238 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(238).unwrap(),
            listen_period: 247 as ms,
        });

        let mut node_239 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(239).unwrap(),
            listen_period: 248 as ms,
        });

        let mut node_240 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(240).unwrap(),
            listen_period: 249 as ms,
        });

        let mut node_241 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(241).unwrap(),
            listen_period: 250 as ms,
        });

        let mut node_242 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(242).unwrap(),
            listen_period: 251 as ms,
        });

        let mut node_243 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(243).unwrap(),
            listen_period: 252 as ms,
        });

        let mut node_244 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(244).unwrap(),
            listen_period: 253 as ms,
        });

        let mut node_245 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(245).unwrap(),
            listen_period: 254 as ms,
        });

        let mut node_246 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(246).unwrap(),
            listen_period: 255 as ms,
        });

        let mut node_247 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(247).unwrap(),
            listen_period: 256 as ms,
        });

        let mut node_248 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(248).unwrap(),
            listen_period: 257 as ms,
        });

        let mut node_249 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(249).unwrap(),
            listen_period: 258 as ms,
        });

        let mut node_250 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(250).unwrap(),
            listen_period: 259 as ms,
        });

        let mut node_251 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(251).unwrap(),
            listen_period: 260 as ms,
        });

        let mut node_252 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(252).unwrap(),
            listen_period: 261 as ms,
        });

        let mut node_253 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(253).unwrap(),
            listen_period: 262 as ms,
        });

        let mut node_254 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(254).unwrap(),
            listen_period: 263 as ms,
        });

        let mut node_255 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(255).unwrap(),
            listen_period: 264 as ms,
        });

        let _ = node_1.send_to_exact(
            NodeString::from_iter("This is the message from node 1".chars()).into_bytes(),
            ExactAddressType::try_from(2).unwrap(),
            LifeTimeType::try_from(10).unwrap(),
            true,
        );

        network_simulator.start_simulation_thread();

        let start_time = Instant::now();

        loop {
            let current_time = Instant::now().duration_since(start_time).as_millis() as ms;

            let _ = node_1.update(&mut modem_1, current_time);
            let _ = node_2.update(&mut modem_2, current_time);
            let _ = node_3.update(&mut modem_3, current_time);
            let _ = node_4.update(&mut modem_4, current_time);
            let _ = node_5.update(&mut modem_5, current_time);
            let _ = node_6.update(&mut modem_6, current_time);
            let _ = node_7.update(&mut modem_7, current_time);
            let _ = node_8.update(&mut modem_8, current_time);
            let _ = node_9.update(&mut modem_9, current_time);
            let _ = node_10.update(&mut modem_10, current_time);
            let _ = node_11.update(&mut modem_11, current_time);
            let _ = node_12.update(&mut modem_12, current_time);
            let _ = node_13.update(&mut modem_13, current_time);
            let _ = node_14.update(&mut modem_14, current_time);
            let _ = node_15.update(&mut modem_15, current_time);
            let _ = node_16.update(&mut modem_16, current_time);
            let _ = node_17.update(&mut modem_17, current_time);
            let _ = node_18.update(&mut modem_18, current_time);
            let _ = node_19.update(&mut modem_19, current_time);
            let _ = node_20.update(&mut modem_20, current_time);
            let _ = node_21.update(&mut modem_21, current_time);
            let _ = node_22.update(&mut modem_22, current_time);
            let _ = node_23.update(&mut modem_23, current_time);
            let _ = node_24.update(&mut modem_24, current_time);
            let _ = node_25.update(&mut modem_25, current_time);
            let _ = node_26.update(&mut modem_26, current_time);
            let _ = node_27.update(&mut modem_27, current_time);
            let _ = node_28.update(&mut modem_28, current_time);
            let _ = node_29.update(&mut modem_29, current_time);
            let _ = node_30.update(&mut modem_30, current_time);
            let _ = node_31.update(&mut modem_31, current_time);
            let _ = node_32.update(&mut modem_32, current_time);
            let _ = node_33.update(&mut modem_33, current_time);
            let _ = node_34.update(&mut modem_34, current_time);
            let _ = node_35.update(&mut modem_35, current_time);
            let _ = node_36.update(&mut modem_36, current_time);
            let _ = node_37.update(&mut modem_37, current_time);
            let _ = node_38.update(&mut modem_38, current_time);
            let _ = node_39.update(&mut modem_39, current_time);
            let _ = node_40.update(&mut modem_40, current_time);
            let _ = node_41.update(&mut modem_41, current_time);
            let _ = node_42.update(&mut modem_42, current_time);
            let _ = node_43.update(&mut modem_43, current_time);
            let _ = node_44.update(&mut modem_44, current_time);
            let _ = node_45.update(&mut modem_45, current_time);
            let _ = node_46.update(&mut modem_46, current_time);
            let _ = node_47.update(&mut modem_47, current_time);
            let _ = node_48.update(&mut modem_48, current_time);
            let _ = node_49.update(&mut modem_49, current_time);
            let _ = node_50.update(&mut modem_50, current_time);
            let _ = node_51.update(&mut modem_51, current_time);
            let _ = node_52.update(&mut modem_52, current_time);
            let _ = node_53.update(&mut modem_53, current_time);
            let _ = node_54.update(&mut modem_54, current_time);
            let _ = node_55.update(&mut modem_55, current_time);
            let _ = node_56.update(&mut modem_56, current_time);
            let _ = node_57.update(&mut modem_57, current_time);
            let _ = node_58.update(&mut modem_58, current_time);
            let _ = node_59.update(&mut modem_59, current_time);
            let _ = node_60.update(&mut modem_60, current_time);
            let _ = node_61.update(&mut modem_61, current_time);
            let _ = node_62.update(&mut modem_62, current_time);
            let _ = node_63.update(&mut modem_63, current_time);
            let _ = node_64.update(&mut modem_64, current_time);
            let _ = node_65.update(&mut modem_65, current_time);
            let _ = node_66.update(&mut modem_66, current_time);
            let _ = node_67.update(&mut modem_67, current_time);
            let _ = node_68.update(&mut modem_68, current_time);
            let _ = node_69.update(&mut modem_69, current_time);
            let _ = node_70.update(&mut modem_70, current_time);
            let _ = node_71.update(&mut modem_71, current_time);
            let _ = node_72.update(&mut modem_72, current_time);
            let _ = node_73.update(&mut modem_73, current_time);
            let _ = node_74.update(&mut modem_74, current_time);
            let _ = node_75.update(&mut modem_75, current_time);
            let _ = node_76.update(&mut modem_76, current_time);
            let _ = node_77.update(&mut modem_77, current_time);
            let _ = node_78.update(&mut modem_78, current_time);
            let _ = node_79.update(&mut modem_79, current_time);
            let _ = node_80.update(&mut modem_80, current_time);
            let _ = node_81.update(&mut modem_81, current_time);
            let _ = node_82.update(&mut modem_82, current_time);
            let _ = node_83.update(&mut modem_83, current_time);
            let _ = node_84.update(&mut modem_84, current_time);
            let _ = node_85.update(&mut modem_85, current_time);
            let _ = node_86.update(&mut modem_86, current_time);
            let _ = node_87.update(&mut modem_87, current_time);
            let _ = node_88.update(&mut modem_88, current_time);
            let _ = node_89.update(&mut modem_89, current_time);
            let _ = node_90.update(&mut modem_90, current_time);
            let _ = node_91.update(&mut modem_91, current_time);
            let _ = node_92.update(&mut modem_92, current_time);
            let _ = node_93.update(&mut modem_93, current_time);
            let _ = node_94.update(&mut modem_94, current_time);
            let _ = node_95.update(&mut modem_95, current_time);
            let _ = node_96.update(&mut modem_96, current_time);
            let _ = node_97.update(&mut modem_97, current_time);
            let _ = node_98.update(&mut modem_98, current_time);
            let _ = node_99.update(&mut modem_99, current_time);
            let _ = node_100.update(&mut modem_100, current_time);
            let _ = node_101.update(&mut modem_101, current_time);
            let _ = node_102.update(&mut modem_102, current_time);
            let _ = node_103.update(&mut modem_103, current_time);
            let _ = node_104.update(&mut modem_104, current_time);
            let _ = node_105.update(&mut modem_105, current_time);
            let _ = node_106.update(&mut modem_106, current_time);
            let _ = node_107.update(&mut modem_107, current_time);
            let _ = node_108.update(&mut modem_108, current_time);
            let _ = node_109.update(&mut modem_109, current_time);
            let _ = node_110.update(&mut modem_110, current_time);
            let _ = node_111.update(&mut modem_111, current_time);
            let _ = node_112.update(&mut modem_112, current_time);
            let _ = node_113.update(&mut modem_113, current_time);
            let _ = node_114.update(&mut modem_114, current_time);
            let _ = node_115.update(&mut modem_115, current_time);
            let _ = node_116.update(&mut modem_116, current_time);
            let _ = node_117.update(&mut modem_117, current_time);
            let _ = node_118.update(&mut modem_118, current_time);
            let _ = node_119.update(&mut modem_119, current_time);
            let _ = node_120.update(&mut modem_120, current_time);
            let _ = node_121.update(&mut modem_121, current_time);
            let _ = node_122.update(&mut modem_122, current_time);
            let _ = node_123.update(&mut modem_123, current_time);
            let _ = node_124.update(&mut modem_124, current_time);
            let _ = node_125.update(&mut modem_125, current_time);
            let _ = node_126.update(&mut modem_126, current_time);
            let _ = node_127.update(&mut modem_127, current_time);
            let _ = node_128.update(&mut modem_128, current_time);
            let _ = node_129.update(&mut modem_129, current_time);
            let _ = node_130.update(&mut modem_130, current_time);
            let _ = node_131.update(&mut modem_131, current_time);
            let _ = node_132.update(&mut modem_132, current_time);
            let _ = node_133.update(&mut modem_133, current_time);
            let _ = node_134.update(&mut modem_134, current_time);
            let _ = node_135.update(&mut modem_135, current_time);
            let _ = node_136.update(&mut modem_136, current_time);
            let _ = node_137.update(&mut modem_137, current_time);
            let _ = node_138.update(&mut modem_138, current_time);
            let _ = node_139.update(&mut modem_139, current_time);
            let _ = node_140.update(&mut modem_140, current_time);
            let _ = node_141.update(&mut modem_141, current_time);
            let _ = node_142.update(&mut modem_142, current_time);
            let _ = node_143.update(&mut modem_143, current_time);
            let _ = node_144.update(&mut modem_144, current_time);
            let _ = node_145.update(&mut modem_145, current_time);
            let _ = node_146.update(&mut modem_146, current_time);
            let _ = node_147.update(&mut modem_147, current_time);
            let _ = node_148.update(&mut modem_148, current_time);
            let _ = node_149.update(&mut modem_149, current_time);
            let _ = node_150.update(&mut modem_150, current_time);
            let _ = node_151.update(&mut modem_151, current_time);
            let _ = node_152.update(&mut modem_152, current_time);
            let _ = node_153.update(&mut modem_153, current_time);
            let _ = node_154.update(&mut modem_154, current_time);
            let _ = node_155.update(&mut modem_155, current_time);
            let _ = node_156.update(&mut modem_156, current_time);
            let _ = node_157.update(&mut modem_157, current_time);
            let _ = node_158.update(&mut modem_158, current_time);
            let _ = node_159.update(&mut modem_159, current_time);
            let _ = node_160.update(&mut modem_160, current_time);
            let _ = node_161.update(&mut modem_161, current_time);
            let _ = node_162.update(&mut modem_162, current_time);
            let _ = node_163.update(&mut modem_163, current_time);
            let _ = node_164.update(&mut modem_164, current_time);
            let _ = node_165.update(&mut modem_165, current_time);
            let _ = node_166.update(&mut modem_166, current_time);
            let _ = node_167.update(&mut modem_167, current_time);
            let _ = node_168.update(&mut modem_168, current_time);
            let _ = node_169.update(&mut modem_169, current_time);
            let _ = node_170.update(&mut modem_170, current_time);
            let _ = node_171.update(&mut modem_171, current_time);
            let _ = node_172.update(&mut modem_172, current_time);
            let _ = node_173.update(&mut modem_173, current_time);
            let _ = node_174.update(&mut modem_174, current_time);
            let _ = node_175.update(&mut modem_175, current_time);
            let _ = node_176.update(&mut modem_176, current_time);
            let _ = node_177.update(&mut modem_177, current_time);
            let _ = node_178.update(&mut modem_178, current_time);
            let _ = node_179.update(&mut modem_179, current_time);
            let _ = node_180.update(&mut modem_180, current_time);
            let _ = node_181.update(&mut modem_181, current_time);
            let _ = node_182.update(&mut modem_182, current_time);
            let _ = node_183.update(&mut modem_183, current_time);
            let _ = node_184.update(&mut modem_184, current_time);
            let _ = node_185.update(&mut modem_185, current_time);
            let _ = node_186.update(&mut modem_186, current_time);
            let _ = node_187.update(&mut modem_187, current_time);
            let _ = node_188.update(&mut modem_188, current_time);
            let _ = node_189.update(&mut modem_189, current_time);
            let _ = node_190.update(&mut modem_190, current_time);
            let _ = node_191.update(&mut modem_191, current_time);
            let _ = node_192.update(&mut modem_192, current_time);
            let _ = node_193.update(&mut modem_193, current_time);
            let _ = node_194.update(&mut modem_194, current_time);
            let _ = node_195.update(&mut modem_195, current_time);
            let _ = node_196.update(&mut modem_196, current_time);
            let _ = node_197.update(&mut modem_197, current_time);
            let _ = node_198.update(&mut modem_198, current_time);
            let _ = node_199.update(&mut modem_199, current_time);
            let _ = node_200.update(&mut modem_200, current_time);
            let _ = node_201.update(&mut modem_201, current_time);
            let _ = node_202.update(&mut modem_202, current_time);
            let _ = node_203.update(&mut modem_203, current_time);
            let _ = node_204.update(&mut modem_204, current_time);
            let _ = node_205.update(&mut modem_205, current_time);
            let _ = node_206.update(&mut modem_206, current_time);
            let _ = node_207.update(&mut modem_207, current_time);
            let _ = node_208.update(&mut modem_208, current_time);
            let _ = node_209.update(&mut modem_209, current_time);
            let _ = node_210.update(&mut modem_210, current_time);
            let _ = node_211.update(&mut modem_211, current_time);
            let _ = node_212.update(&mut modem_212, current_time);
            let _ = node_213.update(&mut modem_213, current_time);
            let _ = node_214.update(&mut modem_214, current_time);
            let _ = node_215.update(&mut modem_215, current_time);
            let _ = node_216.update(&mut modem_216, current_time);
            let _ = node_217.update(&mut modem_217, current_time);
            let _ = node_218.update(&mut modem_218, current_time);
            let _ = node_219.update(&mut modem_219, current_time);
            let _ = node_220.update(&mut modem_220, current_time);
            let _ = node_221.update(&mut modem_221, current_time);
            let _ = node_222.update(&mut modem_222, current_time);
            let _ = node_223.update(&mut modem_223, current_time);
            let _ = node_224.update(&mut modem_224, current_time);
            let _ = node_225.update(&mut modem_225, current_time);
            let _ = node_226.update(&mut modem_226, current_time);
            let _ = node_227.update(&mut modem_227, current_time);
            let _ = node_228.update(&mut modem_228, current_time);
            let _ = node_229.update(&mut modem_229, current_time);
            let _ = node_230.update(&mut modem_230, current_time);
            let _ = node_231.update(&mut modem_231, current_time);
            let _ = node_232.update(&mut modem_232, current_time);
            let _ = node_233.update(&mut modem_233, current_time);
            let _ = node_234.update(&mut modem_234, current_time);
            let _ = node_235.update(&mut modem_235, current_time);
            let _ = node_236.update(&mut modem_236, current_time);
            let _ = node_237.update(&mut modem_237, current_time);
            let _ = node_238.update(&mut modem_238, current_time);
            let _ = node_239.update(&mut modem_239, current_time);
            let _ = node_240.update(&mut modem_240, current_time);
            let _ = node_241.update(&mut modem_241, current_time);
            let _ = node_242.update(&mut modem_242, current_time);
            let _ = node_243.update(&mut modem_243, current_time);
            let _ = node_244.update(&mut modem_244, current_time);
            let _ = node_245.update(&mut modem_245, current_time);
            let _ = node_246.update(&mut modem_246, current_time);
            let _ = node_247.update(&mut modem_247, current_time);
            let _ = node_248.update(&mut modem_248, current_time);
            let _ = node_249.update(&mut modem_249, current_time);
            let _ = node_250.update(&mut modem_250, current_time);
            let _ = node_251.update(&mut modem_251, current_time);
            let _ = node_252.update(&mut modem_252, current_time);
            let _ = node_253.update(&mut modem_253, current_time);
            let _ = node_254.update(&mut modem_254, current_time);
            let _ = node_255.update(&mut modem_255, current_time);

            if current_time >= 300 as ms {
                if let Some(message) = node_2.receive() {
                    let expected = NodeString::from_iter("This is the message from node 1".chars());
                    let got = NodeString::from_iter(message.data.iter().map(|c| *c as char));

                    assert!(got.starts_with(expected.as_str()));

                    break;
                }
            }

            if current_time >= 1000 as ms {
                panic!("Simulation timeout");
            }
        }

        network_simulator.stop_simulation_thread();
    }
}
