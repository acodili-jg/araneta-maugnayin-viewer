use html::text_content::builders::OrderedListBuilder;

use crate::{
    MGA_LARANGAN,
    abbr,
    build_html,
    format::typo,
    html,
    page::{BuildContext, UnpackedBuildContext},
    quote,
};

#[inline]
pub fn bungadsabi(mut context: BuildContext<'_>) {
    page_ii(context.unpack());
    page_iii(context.unpack());
    page_iv(context.unpack());
    page_v(context.unpack());
}

fn page_ii(context: UnpackedBuildContext<'_>) {
    build_html!(context.builder;
        {h1 id("bungadsipi") span id("page-ii") "Bungadsabi"}

        {p id(context.paragraph.increment())
        "Ang " {b "Palagawaang Araneta-" (abbr!(PLPA)) " sa Katawagan para "
        "sa Salipat ng Aghimuan sa Kabukiran"} " ay isang palagawaan ng "
        "sabalak-wika na tinutustusan ng " (for (0, 1, 2, 3) in abbr!(PLPA <>))
        " sa ilalim ng Panukala Blg. " {
            span (abbr!(NSDB_GAUF)) " 7706 Spi" lang("en-PH") translate(false)
        } ". Bilang panimulang hakbang ito ay nagtipon ng siyam (9) na batayan "
        "at magkakaugnay na may 1000-pasok na talasalitaan ng mga tawag at "
        "pahayag sa sumusunod na sinatanging aghimuaning larangan:"}

        {ol [|mut builder: OrderedListBuilder| {
            for larangan in MGA_LARANGAN {
                builder.push(html!(li a
                    "" (larangan.0)
                    href(larangan.2)
                ).build());
            }
            builder
        }]}

        {p id(context.paragraph.increment())
        "Lahat ng mga nangaunang talasalitaan sa Pilipino, pati na yaong sa "
        {span
            [in typo("maugnaying", "Maugnaying")] " Pilipino" translate(false)
        } " na ihinanda ng " {b "Lupon sa Agham"} ", ay naglalaman ng halos "
        "lahatang tawag at pahayag lamang sa agham; walang isa man na "
        "nagbibigay ng tiyak at maayos na mga salitang-pasok tulad ng makikita "
        "sa siyam na talasalitaang ihinanda ngayon ng Palagawaang ito. Ang "
        "siyam na talasalitaan ay panungkulaning sinatungo at maaring gamitin "
        "sa paghahanda ng sapat na mga aklat-aralan para sa lahat ng aghimuing "
        "kaaralan sa dalubhasaan na ibinigay sa " {b "Takaran ng Pamantasang "
        "Gregorio Araneta"} " gayundin para sa mga katulad na kaaralan sa ibang"
        " mga dalubhasaan at pamantasan."}

        {p id(context.paragraph.increment())
        "Ang Sabalak-Angkan ay hindi pangunahing pantalaaralang larangan "
        "ngunit isinima ito bilang hiwalay na panukala dahil sa napapanahon at "
        "dahil sa ang pagkamabisa ng maugnaying Pilipino ay unang naipakilala "
        "sa larangang ito."}
    );
}

fn page_iii(context: UnpackedBuildContext<'_>) {
    build_html!(context.builder;
        {p id(context.paragraph.increment())
        span id("page-iii") "Sa lahatang ayos ng palagawaan, ang bawat isa "
        "sa siyam na guro ng Kagawaran ng Aghimuing Pilipino ng Pamantasan na "
        "may kasanayan at may karanasan ay nakipagtulungan sa isang may "
        "kakayahan at karanasang guro ng bawat aghimuanin o pang-agham na "
        "dalubhasaan ng " (abbr!(TGPA)) ". Ang bawat ganyang koponan ay "
        "nagtipon, nagsuri ng kahulugan, nagsapamantayan at sumbok sa mga tawag"
        " at pahayag ng kaukulang larangan ng koponan."}

        {p id(context.paragraph.increment())
        "Ang gawain ay sinimulan noong " {time date_time("1978-02-16")
        "Pebrero 16, 1978"} " at natapos noong " {time date_time("1978-07-15")
        "Hulyo 15, 1978"} "."}

        {p id(context.paragraph.increment())
        "Ang mga manunungkulan at manunupad ay dumaraing sa kakulangan ng "
        "mga aklat at ibang lathalaing nakasulat sa aghimuing Pilipino. Ito ay "
        "dahil sa kakulangan ng maayos at makatwirang talasalitaan sa iba't "
        "ibang larangan. Sa ngayon, wala pang pampaturuang tatagin, maliban sa "
        "Takaran ng Pamantasang Gregoria Araneta, na nakapaglakip ng "
        "kadalubhasaan sa wika at sa aghimuan at humarap upang tustusan ang "
        "mahalgang pangangailangang ito. Ang iba ay naniniwala na ang mga "
        "aghimuing sansalitaan ay likas na sisipot sa kaukulang panahon. "
        "Maaring gayon nga sa katagalan, ngunit sa haba ng kakailanganing "
        "panahon ay magiging dikatanggap-tanggap; ito ay maaring masukat sa mga"
        " dantaon sa halip sa mga pampaaralang hatintaon."}

        {p id(context.paragraph.increment())
        "Gumagamit ang Pamamaraang " {span translate(false) "Maugnayin"}
        " sa mga hangong salita batay sa mga salitang-ugat, panlapi, panambal "
        "at palabuuan na dati nang alam, o madaling maipaliliwanag, sa kalastao"
        " sa kabukiran ng bansang ito na may matibay na sanligan ng wikang "
        "Samupuluanin o Malayo-Polinesiko. Ang maugnaying pamamaraan ay "
        "salungat sa walang-taros na paggamit ng mga salitang hiram, lalo na "
        "yaong naghahalo ng maraming panambal na Griyego, at Latin na hindi "
        "madaling maipaliwanag sa di-katutubong nagsasalita ng mga wikang "
        "Yurophin. Gayunman, ang mga aghamaing ngalan ng mga halaman at hayop "
        "(hbw: " {b lang("mul") "Terminalia catappa"} " para sa punong talisay)"
        " ay mananatili upang hindi magambala ang kasiyangaan."}
    );
}

fn page_iv(context: UnpackedBuildContext<'_>) {
    build_html!(context.builder;
        {p id(context.paragraph.increment())
        "Inaasahan na mula ngayon ang mga tapos sa Sakahan at mga kaugnay na"
        " kaaralan, na gumamit ng mga " span id("page-iv") "aklat-aralan na "
        "nakasulat sa aghimuing Pilipino, na ihinanda ayon sa Pamamaraang "
        {span translate(false) "Maugnayin"} ", tulad ng ipinaliwanag sa itaas, "
        "ay makakatuklas na maaring ilipat ang aghimuan sa lakastao sa mga "
        "kabukiran."}

        {p id(context.paragraph.increment())
        "May apat na katangian ang siyam na talasalitaan ng Palagawaang ito "
        "na magpapagaan sa kanilang salagom sa kabuuan ng mga katawagang "
        {span "maugnayin" translate(false)} "."}

        {p id(context.paragraph.increment())
        "Una, sila ay maugnayin sa mga salitang nakapasok na sa " (quote!(
        "Maugnaying Talasalitaang Pang-agham, Ingles Pilipino" translate(false)
        )) " na inilathala ng " {b "Lupon sa Agham"} " na itinaguyod ng "
        (abbr!(UNESCO lang("en"))) " noong " {time date_time("1969") "1969"} "."
        " Nagtataglay ng mga 7,500 salita, tawag at pahayag ang lahatang "
        "talasalitaang pang-agham na ito."}

        {p id(context.paragraph.increment())
        "Pangalawa, ang unang 350 mga salitang-pasok sa bawat isa ng siyam "
        "na muntaklat ay magkakaisa, upang mabigyan ang lahat ng siyam na "
        "panungkulaning larangan ng magkatulad na butod ng mga batayang tawag "
        "na pang-agham."}

        {p id(context.paragraph.increment())
        "Pangatlo, sa 350 salitang-pasok, 200 " [in typo("and", "ang")]
        " mga " (quote!("salitang susi")) " na sa Paamaraang Maugnayin ay "
        "kakailangang ituro sa lahat ng mga bata sa mababang paaralan sa una at"
        " ikalawang baitang. Kasama rito ang mga batayang dalumat tulad ng "
        "bilang (" {i lang("en") "number"} "), dami ("
        {i lang("en") "quantity"} "), haba (" {i lang("en") "length"} "), atb. "
        "Maraming mga tawag ang napasama sa mga " (quote!("salitang susi"))
        " hindi lamang sa kanilang kahulugan kundi sa, dahilang sila ay "
        "maliwanag na naglalarawan ng mga tiyak na pamamaraan ng palabuuan ng "
        "salita sa Pilipino. Sa pagsasanay sa mga salitang yaon sa mga murang "
        "gulang, lagi nang matatanim sa isip ng mga mag-aaral sa mababang "
        "paaralan ang mga kaukulang palabuuan ng mga salita."}

        {p id(context.paragraph.increment())
        "Pang-apat, lahat ng siyam na talasalitaan ay maugnayin sa may "
        "humigit-kumulang sa 30,000 " {span "maugnayin" translate(false)} "g "
        "tawag at tumpak na paggamit na nakapanid sa "
        {span (for (0, 1) in abbr!(LUNSAG <{b}>)) translate(false) } ", mga "
        "talaturuang panid na sinisimpan ng Kagawaran ng Aghimuing Pilipino ng pamantasang ito." }
    );
}

fn page_v(context: UnpackedBuildContext<'_>) {
    build_html!(context.builder;
        {p id(context.paragraph.increment())
        span id("page-v") "Ang isip ng tao ay mabilis na natututo, "
        "nakapangangatwiran nang mabuti at nakapag-iisip nang lalong malikhain "
        "kapag maraming landas ng pagkakaugnayan sa mga salita, tawag at "
        "pahayag ng wikang ginagamit. Sagitan ng hakbang na ito sa "
        {b "Palagawaang Araneta " (abbr!(PLPA)) " sa Katawagan para sa Salipat "
        "ng Aghimuan sa Kabukiran,"} " umaasa kami na nakasulong kami nang "
        "malaki tungo sa malawak na saunlad ng agham at aghimuan sa bansang "
        "ito."}

        {ul li
            {address "Gonsalo del Rosario"}
            {ul
                {li "Pamuno ng Panukala"}
                {li time date_time("1978-07-15") "Hulyo 15, 1978"}
            }
        }
    );
}
