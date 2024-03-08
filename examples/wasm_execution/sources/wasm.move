// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

module rooch_examples::wasm_execution {
   use moveos_std::context::{Context};
   use moveos_std::wasm;
   use std::debug;
   use std::vector;

   entry public fun run(ctx: &Context) {
      let wasm_bytes: vector<u8> = x"0061736d0100000001110460017f017f60000060017f006000017f030807000100000002030405017001020205060101800280020608017f0141808c040b076907066d656d6f727902000a746573745f616c6c6f6300030b5f696e697469616c697a650001195f5f696e6469726563745f66756e6374696f6e5f7461626c65010009737461636b5361766500060c737461636b526573746f726500050a737461636b416c6c6f6300040907010041010b01010ae329074901027f4180082802002201200041076a41787122026a2100024020024100200020014d1b45044020003f004110744d0d010b4184084130360200417f0f0b418008200036020020010b02000b7a01037f0240024020002201410371450d0020012d000045044041000f0b0340200141016a2201410371450d0120012d00000d000b0c010b03402001220241046a210120022802002203417f73200341818284086b7141808182847871450d000b03402002220141016a210220012d00000d000b0b200120006b0bfb27010d7f230041106b220a2400200a42adc2c183d7cc9b32370008200a41086a1002200010026a2100230041106b220b24000240024002400240024002400240024002400240200041f4014d0440418808280200220341102000410b6a41f803712000410b491b22064103762200762201410371044002402001417f7341017120006a2202410374220041b0086a2201200041b8086a280200220028020822054604404188082003417e200277713602000c010b2005200136020c200120053602080b200041086a2101200020024103742202410372360204200020026a220020002802044101723602040c0b0b200641900828020022084d0d0120010440024041022000742202410020026b72200120007471682201410374220041b0086a2202200041b8086a280200220028020822054604404188082003417e2001777122033602000c010b2005200236020c200220053602080b20002006410372360204200020066a22042001410374220120066b2205410172360204200020016a200536020020080440200841787141b0086a2101419c082802002102027f20034101200841037674220771450440418808200320077236020020010c010b20012802080b2103200120023602082003200236020c2002200136020c200220033602080b200041086a2101419c08200436020041900820053602000c0b0b418c08280200220c450d01200c6841027441b80a6a280200220228020441787120066b210420022100034002402000280210220145044020002802142201450d010b200128020441787120066b22002004200020044922001b21042001200220001b2102200121000c010b0b200228021821092002200228020c22014704404198082802001a20022802082200200136020c200120003602080c0a0b20022802142200047f200241146a0520022802102200450d03200241106a0b21050340200521072000220141146a2105200028021422000d00200141106a2105200128021022000d000b200741003602000c090b417f2106200041bf7f4b0d002000410b6a22004178712106418c082802002207450d00410020066b2104024002400240027f41002006418002490d001a411f200641ffffff074b0d001a2006412620004108766722006b7641017120004101746b413e6a0b220841027441b80a6a28020022004504400c010b2006411920084101766b41002008411f471b74210203400240200028020441787120066b220320044f0d0020002105200322040d0041002104200021010c030b200120002802142203200320002002411d764104716a2802102200461b200120031b21012002410174210220000d000b0b20012005724504404100210541022008742200410020006b722007712200450d0320006841027441b80a6a28020021010b2001450d010b0340200128020441787120066b220220044921002002200420001b21042001200520001b210520012802102200047f20000520012802140b22010d000b0b2005450d00200441900828020020066b4f0d00200528021821082005200528020c22014704404198082802001a20052802082200200136020c200120003602080c080b20052802142200047f200541146a0520052802102200450d03200541106a0b21020340200221032000220141146a2102200028021422000d00200141106a2102200128021022000d000b200341003602000c070b200641900828020022054d0440419c0828020021010240200520066b220041104f0440200120066a22022000410172360204200120056a2000360200200120064103723602040c010b20012005410372360204200120056a2200200028020441017236020441002102410021000b4190082000360200419c082002360200200141086a21010c090b20064194082802002202490440419408200220066b220136020041a00841a008280200220020066a22023602002002200141017236020420002006410372360204200041086a21010c090b410021012006412f6a2204027f41e00b280200044041e80b2802000c010b41ec0b427f37020041e40b4280a0808080800437020041e00b200b410c6a41707141d8aad5aa057336020041f40b410036020041c40b41003602004180200b22006a2203410020006b220771220020064d0d0841c00b2802002205044041b80b280200220820006a220920084d2005200949720d090b024041c40b2d0000410471450440024002400240024041a0082802002205044041c80b210103402005200128020022084f0440200820012802046a20054b0d030b200128020822010d000b0b410010002202417f460d032000210341e40b280200220141016b22052002710440200020026b200220056a410020016b716a21030b200320064d0d0341c00b2802002201044041b80b280200220520036a220720054d2001200749720d040b2003100022012002470d010c050b200320026b200771220310002202200128020020012802046a460d01200221010b2001417f460d01200641306a20034d0440200121020c040b41e80b2802002202200420036b6a410020026b7122021000417f460d01200220036a2103200121020c030b2002417f470d020b41c40b41c40b2802004104723602000b200010002202417f46410010002200417f4672200020024d720d05200020026b2203200641286a4d0d050b41b80b41b80b28020020036a220036020041bc0b280200200049044041bc0b20003602000b024041a0082802002204044041c80b21010340200220012802002200200128020422056a460d02200128020822010d000b0c040b41980828020022004100200020024d1b45044041980820023602000b4100210141cc0b200336020041c80b200236020041a808417f36020041ac0841e00b28020036020041d40b410036020003402001410374220041b8086a200041b0086a2205360200200041bc086a2005360200200141016a22014120470d000b419408200341286b2200417820026b41077122016b220536020041a008200120026a220136020020012005410172360204200020026a412836020441a40841f00b2802003602000c040b200220044d200020044b720d02200128020c4108710d022001200320056a36020441a0082004417820046b41077122006a220136020041940841940828020020036a220220006b220036020020012000410172360204200220046a412836020441a40841f00b2802003602000c030b410021010c060b410021010c040b41980828020020024b044041980820023602000b200220036a210041c80b21010240034020002001280200470440200128020822010d010c020b0b20012d000c410871450d030b41c80b2101034002402004200128020022004f0440200020012802046a220520044b0d010b200128020821010c010b0b419408200341286b2200417820026b41077122016b220736020041a008200120026a220136020020012007410172360204200020026a412836020441a40841f00b28020036020020042005412720056b4107716a412f6b22002000200441106a491b2200411b360204200041d00b290200370210200041c80b29020037020841d00b200041086a36020041cc0b200336020041c80b200236020041d40b4100360200200041186a2101034020014107360204200141086a210d200141046a2101200d2005490d000b20002004460d0020002000280204417e713602042004200020046b220241017236020420002002360200027f200241ff014d0440200241787141b0086a2101027f41880828020022004101200241037674220271450440418808200020027236020020010c010b20012802080b2100200120043602082000200436020c410c210241080c010b411f2101200241ffffff074d04402002412620024108766722006b7641017120004101746b413e6a21010b2004200136021c20044200370210200141027441b80a6a210002400240418c0828020022054101200174220371450440418c082003200572360200200020043602000c010b2002411920014101766b41002001411f471b742101200028020021050340200522002802044178712002460d022001411d76210520014101742101200020054104716a220328021022050d000b200320043602100b2004200036021841082102200422002101410c0c010b20002802082201200436020c20002004360208200420013602084100210141182102410c0b20046a2000360200200220046a20013602000b419408280200220020064d0d00419408200020066b220136020041a00841a008280200220020066a22023602002002200141017236020420002006410372360204200041086a21010c040b4184084130360200410021010c030b200120023602002001200128020420036a3602042002417820026b4107716a220820064103723602042000417820006b4107716a2203200620086a22046b2107024041a008280200200346044041a008200436020041940841940828020020076a2200360200200420004101723602040c010b419c082802002003460440419c08200436020041900841900828020020076a220036020020042000410172360204200020046a20003602000c010b20032802042201410371410146044020014178712109200328020c21020240200141ff014d0440200328020822002002460440418808418808280200417e200141037677713602000c020b2000200236020c200220003602080c010b200328021821060240200220034704404198082802001a20032802082200200236020c200220003602080c010b024020032802142201047f200341146a0520032802102201450d01200341106a0b21000340200021052001220241146a2100200128021422010d00200241106a2100200228021022010d000b200541003602000c010b410021020b2006450d000240200328021c220041027441b80a6a220128020020034604402001200236020020020d01418c08418c08280200417e200077713602000c020b20064110411420062802102003461b6a20023602002002450d010b2002200636021820032802102200044020022000360210200020023602180b20032802142200450d0020022000360214200020023602180b200720096a2107200320096a220328020421010b20032001417e7136020420042007410172360204200420076a2007360200200741ff014d0440200741787141b0086a2100027f41880828020022014101200741037674220271450440418808200120027236020020000c010b20002802080b2101200020043602082001200436020c2004200036020c200420013602080c010b411f2102200741ffffff074d04402007412620074108766722006b7641017120004101746b413e6a21020b2004200236021c20044200370210200241027441b80a6a210102400240418c0828020022004101200274220571450440418c082000200572360200200120043602000c010b2007411920024101766b41002002411f471b742102200128020021000340200022012802044178712007460d022002411d76210020024101742102200120004104716a220528021022000d000b200520043602100b200420013602182004200436020c200420043602080c010b20012802082200200436020c20012004360208200441003602182004200136020c200420003602080b200841086a21010c020b02402008450d000240200528021c220041027441b80a6a220228020020054604402002200136020020010d01418c082007417e2000777122073602000c020b20084110411420082802102005461b6a20013602002001450d010b2001200836021820052802102200044020012000360210200020013602180b20052802142200450d0020012000360214200020013602180b02402004410f4d04402005200420066a2200410372360204200020056a220020002802044101723602040c010b20052006410372360204200520066a22032004410172360204200320046a2004360200200441ff014d0440200441787141b0086a2100027f41880828020022014101200441037674220271450440418808200120027236020020000c010b20002802080b2101200020033602082001200336020c2003200036020c200320013602080c010b411f2101200441ffffff074d04402004412620044108766722006b7641017120004101746b413e6a21010b2003200136021c20034200370210200141027441b80a6a21000240024020074101200174220271450440418c08200220077236020020002003360200200320003602180c010b2004411920014101766b41002001411f471b742101200028020021000340200022022802044178712004460d022001411d76210020014101742101200220004104716a220728021022000d000b20072003360210200320023602180b2003200336020c200320033602080c010b20022802082200200336020c20022003360208200341003602182003200236020c200320003602080b200541086a21010c010b02402009450d000240200228021c220041027441b80a6a220528020020024604402005200136020020010d01418c08200c417e200077713602000c020b20094110411420092802102002461b6a20013602002001450d010b2001200936021820022802102200044020012000360210200020013602180b20022802142200450d0020012000360214200020013602180b02402004410f4d04402002200420066a2200410372360204200020026a220020002802044101723602040c010b20022006410372360204200220066a22052004410172360204200420056a200436020020080440200841787141b0086a2100419c082802002101027f41012008410376742207200371450440418808200320077236020020000c010b20002802080b2103200020013602082003200136020c2001200036020c200120033602080b419c08200536020041900820043602000b200241086a21010b200b41106a2400200a41106a240020010b1000230020006b4170712200240020000b0600200024000b040023000b0b0901004181080b020601";
      // create wasm VM instance
      let wasm_instance_id = wasm::create_wasm_instance(ctx, wasm_bytes);
      debug::print(&wasm_instance_id);
      let function_name = b"test_alloc";
      let arg = b"argument1";
      let arg_list = vector::empty<vector<u8>>();
      vector::push_back(&mut arg_list, arg);
      // encode the JSON string to CBOR format
      // pub the bytes data of args to the stack area of the wasm VM memory
      let args_list = wasm::create_wasm_args(wasm_instance_id, function_name, arg_list);
      // call the wasm function (when the calling was stoped, delete the wasm instance)
      // use the data pointer to get the bytes dat of return value
      debug::print(&args_list);
   }
}
