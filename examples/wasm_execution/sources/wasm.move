// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

module rooch_examples::wasm_execution {
   use moveos_std::wasm;
   use std::debug;
   use std::string;
   use std::vector;

   entry public fun run() {
      let wasm_bytes: vector<u8> = x"0061736d0100000001180560017f017f60000060037f7f7f017f60017f006000017f03090800010200000003040405017001020205060101800280020608017f0141808c040b076907066d656d6f727902000a746573745f616c6c6f6300040b5f696e697469616c697a650001195f5f696e6469726563745f66756e6374696f6e5f7461626c65010009737461636b5361766500070c737461636b526573746f726500060a737461636b416c6c6f6300050907010041010b01010ac02a084901027f4180082802002201200041076a41787122026a2100024020024100200020014d1b45044020003f004110744d0d010b4184084130360200417f0f0b418008200036020020010b02000b3301017f20020440200021030340200320012d00003a0000200341016a2103200141016a2101200241016b22020d000b0b20000b7a01037f0240024020002201410371450d0020012d000045044041000f0b0340200141016a2201410371450d0120012d00000d000b0c010b03402001220241046a210120022802002203417f73200341818284086b7141808182847871450d000b03402002220141016a210220012d00000d000b0b200120006b0ba42801127f230041106b220b2400200b42adc2c183d7cc9b32370008200b41086a220f1003221020001003220c6a41016a22112101230041106b220d24000240024002400240024002400240024002400240200141f4014d0440418808280200220441102001410b6a41f803712001410b491b22074103762201762202410371044002402002417f7341017120016a2203410374220141b0086a2202200141b8086a280200220128020822064604404188082004417e200377713602000c010b2006200236020c200220063602080b200141086a2102200120034103742203410372360204200120036a220120012802044101723602040c0b0b200741900828020022094d0d0120020440024041022001742203410020036b72200220017471682202410374220141b0086a2203200141b8086a280200220128020822064604404188082004417e2002777122043602000c010b2006200336020c200320063602080b20012007410372360204200120076a22052002410374220220076b2206410172360204200120026a200636020020090440200941787141b0086a2102419c082802002103027f20044101200941037674220871450440418808200420087236020020020c010b20022802080b2104200220033602082004200336020c2003200236020c200320043602080b200141086a2102419c08200536020041900820063602000c0b0b418c08280200220e450d01200e6841027441b80a6a280200220328020441787120076b210520032101034002402001280210220245044020012802142202450d010b200228020441787120076b22012005200120054922011b21052002200320011b2103200221010c010b0b2003280218210a2003200328020c22024704404198082802001a20032802082201200236020c200220013602080c0a0b20032802142201047f200341146a0520032802102201450d03200341106a0b21060340200621082001220241146a2106200128021422010d00200241106a2106200228021022010d000b200841003602000c090b417f2107200141bf7f4b0d002001410b6a22014178712107418c082802002208450d00410020076b2105024002400240027f41002007418002490d001a411f200741ffffff074b0d001a2007412620014108766722016b7641017120014101746b413e6a0b220941027441b80a6a28020022014504400c010b2007411920094101766b41002009411f471b74210303400240200128020441787120076b220420054f0d0020012106200422050d0041002105200121020c030b200220012802142204200420012003411d764104716a2802102201461b200220041b21022003410174210320010d000b0b20022006724504404100210641022009742201410020016b722008712201450d0320016841027441b80a6a28020021020b2002450d010b0340200228020441787120076b220320054921012003200520011b21052002200620011b210620022802102201047f20010520022802140b22020d000b0b2006450d00200541900828020020076b4f0d00200628021821092006200628020c22024704404198082802001a20062802082201200236020c200220013602080c080b20062802142201047f200641146a0520062802102201450d03200641106a0b21030340200321042001220241146a2103200128021422010d00200241106a2103200228021022010d000b200441003602000c070b200741900828020022064d0440419c0828020021020240200620076b220141104f0440200220076a22032001410172360204200220066a2001360200200220074103723602040c010b20022006410372360204200220066a2201200128020441017236020441002103410021010b4190082001360200419c082003360200200241086a21020c090b20074194082802002203490440419408200320076b220236020041a00841a008280200220120076a22033602002003200241017236020420012007410372360204200141086a21020c090b410021022007412f6a2205027f41e00b280200044041e80b2802000c010b41ec0b427f37020041e40b4280a0808080800437020041e00b200d410c6a41707141d8aad5aa057336020041f40b410036020041c40b41003602004180200b22016a2204410020016b220871220120074d0d0841c00b2802002206044041b80b280200220920016a220a20094d2006200a49720d090b024041c40b2d0000410471450440024002400240024041a0082802002206044041c80b210203402006200228020022094f0440200920022802046a20064b0d030b200228020822020d000b0b410010002203417f460d032001210441e40b280200220241016b22062003710440200120036b200320066a410020026b716a21040b200420074d0d0341c00b2802002202044041b80b280200220620046a220820064d2002200849720d040b2004100022022003470d010c050b200420036b200871220410002203200228020020022802046a460d01200321020b2002417f460d01200741306a20044d0440200221030c040b41e80b2802002203200520046b6a410020036b7122031000417f460d01200320046a2104200221030c030b2003417f470d020b41c40b41c40b2802004104723602000b200110002203417f46410010002201417f4672200120034d720d05200120036b2204200741286a4d0d050b41b80b41b80b28020020046a220136020041bc0b280200200149044041bc0b20013602000b024041a0082802002205044041c80b21020340200320022802002201200228020422066a460d02200228020822020d000b0c040b41980828020022014100200120034d1b45044041980820033602000b4100210241cc0b200436020041c80b200336020041a808417f36020041ac0841e00b28020036020041d40b410036020003402002410374220141b8086a200141b0086a2206360200200141bc086a2006360200200241016a22024120470d000b419408200441286b2201417820036b41077122026b220636020041a008200220036a220236020020022006410172360204200120036a412836020441a40841f00b2802003602000c040b200320054d200120054b720d02200228020c4108710d022002200420066a36020441a0082005417820056b41077122016a220236020041940841940828020020046a220320016b220136020020022001410172360204200320056a412836020441a40841f00b2802003602000c030b410021020c060b410021020c040b41980828020020034b044041980820033602000b200320046a210141c80b21020240034020012002280200470440200228020822020d010c020b0b20022d000c410871450d030b41c80b2102034002402005200228020022014f0440200120022802046a220620054b0d010b200228020821020c010b0b419408200441286b2201417820036b41077122026b220836020041a008200220036a220236020020022008410172360204200120036a412836020441a40841f00b28020036020020052006412720066b4107716a412f6b22012001200541106a491b2201411b360204200141d00b290200370210200141c80b29020037020841d00b200141086a36020041cc0b200436020041c80b200336020041d40b4100360200200141186a2102034020024107360204200241086a2112200241046a210220122006490d000b20012005460d0020012001280204417e713602042005200120056b220341017236020420012003360200027f200341ff014d0440200341787141b0086a2102027f41880828020022014101200341037674220371450440418808200120037236020020020c010b20022802080b2101200220053602082001200536020c410c210341080c010b411f2102200341ffffff074d04402003412620034108766722016b7641017120014101746b413e6a21020b2005200236021c20054200370210200241027441b80a6a210102400240418c0828020022064101200274220471450440418c082004200672360200200120053602000c010b2003411920024101766b41002002411f471b742102200128020021060340200622012802044178712003460d022002411d76210620024101742102200120064104716a220428021022060d000b200420053602100b2005200136021841082103200522012102410c0c010b20012802082202200536020c20012005360208200520023602084100210241182103410c0b20056a2001360200200320056a20023602000b419408280200220120074d0d00419408200120076b220236020041a00841a008280200220120076a22033602002003200241017236020420012007410372360204200141086a21020c040b4184084130360200410021020c030b200220033602002002200228020420046a3602042003417820036b4107716a220920074103723602042001417820016b4107716a2204200720096a22056b2108024041a008280200200446044041a008200536020041940841940828020020086a2201360200200520014101723602040c010b419c082802002004460440419c08200536020041900841900828020020086a220136020020052001410172360204200120056a20013602000c010b2004280204220241037141014604402002417871210a200428020c21030240200241ff014d0440200428020822012003460440418808418808280200417e200241037677713602000c020b2001200336020c200320013602080c010b200428021821070240200320044704404198082802001a20042802082201200336020c200320013602080c010b024020042802142202047f200441146a0520042802102202450d01200441106a0b21010340200121062002220341146a2101200228021422020d00200341106a2101200328021022020d000b200641003602000c010b410021030b2007450d000240200428021c220141027441b80a6a220228020020044604402002200336020020030d01418c08418c08280200417e200177713602000c020b20074110411420072802102004461b6a20033602002003450d010b2003200736021820042802102201044020032001360210200120033602180b20042802142201450d0020032001360214200120033602180b2008200a6a21082004200a6a220428020421020b20042002417e7136020420052008410172360204200520086a2008360200200841ff014d0440200841787141b0086a2101027f41880828020022024101200841037674220371450440418808200220037236020020010c010b20012802080b2102200120053602082002200536020c2005200136020c200520023602080c010b411f2103200841ffffff074d04402008412620084108766722016b7641017120014101746b413e6a21030b2005200336021c20054200370210200341027441b80a6a210202400240418c0828020022014101200374220671450440418c082001200672360200200220053602000c010b2008411920034101766b41002003411f471b742103200228020021010340200122022802044178712008460d022003411d76210120034101742103200220014104716a220628021022010d000b200620053602100b200520023602182005200536020c200520053602080c010b20022802082201200536020c20022005360208200541003602182005200236020c200520013602080b200941086a21020c020b02402009450d000240200628021c220141027441b80a6a220328020020064604402003200236020020020d01418c082008417e2001777122083602000c020b20094110411420092802102006461b6a20023602002002450d010b2002200936021820062802102201044020022001360210200120023602180b20062802142201450d0020022001360214200120023602180b02402005410f4d04402006200520076a2201410372360204200120066a220120012802044101723602040c010b20062007410372360204200620076a22042005410172360204200420056a2005360200200541ff014d0440200541787141b0086a2101027f41880828020022024101200541037674220371450440418808200220037236020020010c010b20012802080b2102200120043602082002200436020c2004200136020c200420023602080c010b411f2102200541ffffff074d04402005412620054108766722016b7641017120014101746b413e6a21020b2004200236021c20044200370210200241027441b80a6a21010240024020084101200274220371450440418c08200320087236020020012004360200200420013602180c010b2005411920024101766b41002002411f471b742102200128020021010340200122032802044178712005460d022002411d76210120024101742102200320014104716a220828021022010d000b20082004360210200420033602180b2004200436020c200420043602080c010b20032802082201200436020c20032004360208200441003602182004200336020c200420013602080b200641086a21020c010b0240200a450d000240200328021c220141027441b80a6a220628020020034604402006200236020020020d01418c08200e417e200177713602000c020b200a41104114200a2802102003461b6a20023602002002450d010b2002200a36021820032802102201044020022001360210200120023602180b20032802142201450d0020022001360214200120023602180b02402005410f4d04402003200520076a2201410372360204200120036a220120012802044101723602040c010b20032007410372360204200320076a22062005410172360204200520066a200536020020090440200941787141b0086a2101419c082802002102027f41012009410376742208200471450440418808200420087236020020010c010b20012802080b2104200120023602082004200236020c2002200136020c200220043602080b419c08200636020041900820053602000b200341086a21020b200d41106a2400200c20022000200c100222006a200f201010021a200020116a41003a0000200b41106a240020000b1000230020006b4170712200240020000b0600200024000b040023000b0b0901004181080b020601";
      // 1. create wasm VM instance (required step)
      let wasm_instance_id = wasm::create_wasm_instance(wasm_bytes);
      debug::print(&wasm_instance_id);

      // 2. encode the JSON string bytes by using CBOR format

      // 3. add the length value of the CBOR data

      // 4. append a NULL byte to data if the generator can process c string

      // 5. put the bytes data of args to the stack area in the wasm VM memory
      let function_name = b"test_alloc";
      let arg = b"argument1";
      let arg_list = vector::empty<vector<u8>>();
      vector::push_back(&mut arg_list, arg);
      let wrapped_args_list = wasm::create_memory_wasm_args(wasm_instance_id, function_name, arg_list);
      debug::print(&wrapped_args_list);

      // 6. call the wasm function (required step)
      // when the calling is finished, release the wasm instance and memory
      let ret_val = wasm::execute_wasm_function(wasm_instance_id, function_name, wrapped_args_list);
      debug::print(&ret_val);

      // 7. get the length of the data

      // 8. read the bytes dat of return value by pointer offset and data length
      let ret_data = wasm::read_data_from_heap(wasm_instance_id, (ret_val as u32), 0);
      let ret_string = string::utf8(ret_data);
      debug::print(&ret_string);

      // 9. release the wasm VM instance (required step)
      wasm::release_wasm_instance(wasm_instance_id);
   }
}
