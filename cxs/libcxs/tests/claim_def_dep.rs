extern crate rusqlite;

use std::env::home_dir;


static ENTRIES: &[[&str;3];6] = &[
    [
        "claim_definition_private::2hoqvcwupRTUNkXn6ArYzs:15",
        r#"{"secret_key":{"p":"139487684743426934866224951962492278976033872927645764870932099401267417300834447210226071235132419566799186258471528658750224371754894423526601605874987628975850529277895758002503639302057158499505184482062019901147451416880932855653178276409188907179487058776296448670567237107274584022583573914693235126821","q":"150848315904778847602727788295995229612801377223526257362482582038708340756470009735494354494609454294515467595648482398436537100160962376767833965508330374282858620557637157382774173692642101254072735113266054904673545355852844559552372533850929673778342367814495098505607753561587539451888902205754598009903"},"secret_key_revocation":null}"#,
        "2017-12-21 23:38:36"
    ],
    [
        "claim_definition::2hoqvcwupRTUNkXn6ArYzs:15",
        r#"{"ref": 15, "seqNo": 209, "txnTime": 1513899516, "type": "108", "origin": "2hoqvcwupRTUNkXn6ArYzs", "signature_type": "CL", "reqId": 1513976457245259312, "data": {"primary": {"rctxt": "15178418908707976396874115890155369378739552844710001287620150293516043143109012669349041097433185985006637619942258175978540027428269463967862486014712935577429209884916439319844881887838005724689572541655179363820202303909255020846738207944149748325401015166973694663513705334001272587139784018617051367282195864236309518887180735447664294744244257623016534969984576531909897381675727144248416907952365426263870687246374199940494051456843185693216994831715033002131331315926761224094035394159492212158361327683840337563628100786666306290246929789056454464994993951075589412237163033252373844501295459179828983092882", "s": "32149049575821945230621749357679852960218975275663660378322434136493234768541978471643370093533144378428310282499891836069077315709802240182563371494910228134461744250834937468570668937610651921421149641961876146548466370852482470627663417472750928321122264102385703719915700203898572896689940513282392144102171236231349816060223945370207116144512166185890414339373942227207962354797987602833067017559740203840221474306826305168649462069199457141160732391024874191093314329416034701048235166161519770903880979479587716201827609950051559446571527547219731053857242710326291179981624058687846557478419856645442048458983", "n": "84165929332010668428460804365759072114509339544585582111239590375390859858393423238072893952739099460949405087844679732888081972605884803743544408738013381203447376624848411124945264559705573882676357134703531618608394413501975796303816844162338449557165477692446549865681396021010317794479826467455363080679992206264412573720421460193952110261769205251411188969108476965744043996499676198251277289188830834203358587140511615307359495384498076560223229007189513019713172290253155835538655896556724062836246614864623813083755500577658609851104269988816266763530745872504932864081861327627122657390784383204817341906901", "rms": "14081999445207369753025452095181614342065215947012176015948691977063552703325940081753475729550579991679021946255784473411092928255132634026697688645887895718497722729538529282283228337413298166770794329658775032983056278220674013655505196986571998836886746636973591030037621930165201103096054737348385601140232674676706299825003827206512254071561729284839075478106754274589540298139980488837998237495311150365306286672353376472956530832088802989883408203894252629390826582429755506350883567784853842215814538626632104161541556480839579139982723474217405283373482287230729509781253700516657378530215992273439037306163", "r": {"address2": "64093835540639134462162324818753842626601309036971270213589138270157612330595530667934181472829269268708885560307381173519247954587237416889078003654170202198855341442601554736298178856328441741127160263636049912654032633645512918413503174357148746399387350726427447554505187715643827586823723171297767637740031657993311453590772023318561239935191576952003614013844162604036638967970578014166330059891148134120361367905041266196482603786759286607791304592464358170677916769912251287183262159484866486586536051541391800024794963792601784116763338284701313179009571830752433717562525315829638297695595980242620305451763", "zip": "24689013608314572092666143639621907208387832838774450125231850774665963408762271716043753404693900913824132036054436149699013251556614233816749351295496920801467985910615015744533376420578222082477898813379526483255520590837998137522261759170294742160254463498301088473062228255212523057199558846388151200324605456416575660089442878855249536551009104571015055792539673817260198054425005892140479628613741214710054783851570515639345034911909045402120167271398375513748040609603283418095438068178795964393518459331195300507900579325412241149879879236524870906791853668676452071468097897591673185667597609894490849395695", "address1": "10640525489609058611507073499505451636035397824762741424923567584484530131259760855832289659842120216110448780159467111494926521603617851264866791548199480509185598583780863077119640812560893079877467830314852385570635787248277924048670143296383726032399844696700699797937835693086719725004035259524793782194143319010620618421167805761691274228173305001364321016019132799913718593586127392608962973860755287269184060807822065989964694270258675336811196612988739180796382127412380225101431466467330046846718290777122701653019610370026040846336893389696883907345026783164674277952616762104745211962290194330822896218734", "state": "23601373530668021977796978437170814743495474976056179436026884354367783961740498805853056593501873750265070941490053385760131868994630211021117002189999239871790952717153479754498268720600513398874750860736119021398469692191533371134873813025901144473502435596924829291361641406563171351335187043571440127776341381919826905267144440570412497187879999878715870372989074930878216932574145987717214336745257446012566382380881582134990042107776224023625137362880135383726853733859511628601779230517443521984689731123584711098304113751207461183612376855631922515175029487930630228402627443619090971882130232343721989739796", "city": "39177307725481428811221074055554872705678123323656544894084055979955911629674496885552755583515750190937363406396782751092881096608571727899413220147090622192475737402500873357662129999843542570060705022054129432372980196262481459959733393110018612899252707162181019641546825674532898354366870699805353271962086049635031735759004587787027830587164258977822026168813860351096625583487512204229054062667688275123922407113117673490539610931959339794566503001590667819528090335699867148230716397846369596353963833729172314405741711135457672500483835825033729325577500866185102493247899544978122839537190533216811071591563"}, "z": "74557446919007877902775531863054122916962666113784506745925109941160685972381043377807207915937932165886923249047051061927504907805355725579162295071326214164974092717299571921963014036435768596805857919630727389668224358322974955468807592755990133611180899812099872121078543730249135946196677825120845812689885445913406825299550461023234438565813765650333663995795788382919224073577891581185264572872434816036420022441842720136797091402157710814892048017911492149070235454328654783437327775249092064050341826809756761002743138215088112506313750438236988044312265809494217149788246156701173874423705983508767201291836"}, "revocation": null}, "identifier": "2hoqvcwupRTUNkXn6ArYzs"}"#,
        "2017-12-22 21:00:57"
    ],
    [
        "my_did::2hoqvcwupRTUNkXn6ArYzs",
        r#"{"did":"2hoqvcwupRTUNkXn6ArYzs","verkey":"vrWGArMA3toVoZrYGSAMjR2i9KjBS66bZWyWuYJJYPf"}"#,
        "2017-12-22 21:00:57"
    ],
    [
        "key::vrWGArMA3toVoZrYGSAMjR2i9KjBS66bZWyWuYJJYPf",
        r#"{"verkey":"vrWGArMA3toVoZrYGSAMjR2i9KjBS66bZWyWuYJJYPf","signkey":"xt19s1sp2UZCGhy9rNyb1FtxdKiDGZZPPWbEpqU41NEy3MmoeUuhq6T8F4JuRPgtj3hoT6BBjgnEF4bbhsR84NH"}"#,
        "2017-12-22 21:00:57"
    ],
    [
        "my_did::5bJqPo8aCWyBwLQosZkJcB",
        r#"{"did":"5bJqPo8aCWyBwLQosZkJcB","verkey":"3W9WGtRowAanh5q6giQrGncZVMvRwPedB9fJAJkAN5Gk"}"#,
        "2018-01-04 22:46:18"
    ],
    [
        "key::3W9WGtRowAanh5q6giQrGncZVMvRwPedB9fJAJkAN5Gk",
        r#"{"verkey":"3W9WGtRowAanh5q6giQrGncZVMvRwPedB9fJAJkAN5Gk","signkey":"xt19s1sp2UZCGhy9rNyb1FtxdKiDGZaRvDuYQ4AKCZCipe1dDms9sw66AKazbdzTSisr1CKjA1r1iDZcjw6pNUY"}"#,
        "2018-01-04 22:46:18"
    ]
];
#[ignore]
#[test]
fn test_putting_claim_def_dependencies() {
    use std::path::Path;
    use rusqlite::Connection;
    let home = home_dir().unwrap();
    let indy = Path::new(".indy_client/wallet/my_real_wallet/sqlite.db");
    let path = home.join(indy);
    let connection = Connection::open(path.as_path()).unwrap();


    for entry in ENTRIES {
        connection.execute("INSERT INTO wallet VALUES (?,?,?)", &[&entry[0].to_string(),&entry[1].to_string(),&entry[2].to_string()]).unwrap();
    }

}