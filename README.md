# CodeWars-Gap-in-Primes-5-kyu---Passed
The prime numbers are not regularly spaced. For example from 2 to 3 the gap is 1. From 3 to 5 the gap is 2. From 7 to 11 it is 4. Between 2 and 50 we have the following pairs of 2-gaps primes: 3-5, 5-7, 11-13, 17-19, 29-31, 41-43

A prime gap of length n is a run of n-1 consecutive composite numbers between two successive primes (see: http://mathworld.wolfram.com/PrimeGaps.html).

We will write a function gap with parameters:

g (integer >= 2) which indicates the gap we are looking for

m (integer > 2) which gives the start of the search (m inclusive)

n (integer >= m) which gives the end of the search (n inclusive)

In the example above gap(2, 3, 50) will return [3, 5] or (3, 5) or {3, 5} which is the first pair between 3 and 50 with a 2-gap.

So this function should return the first pair of two prime numbers spaced with a gap of g between the limits m, n if these numbers exist otherwise `nil or null or None or Nothing (or ... depending on the language).

In such a case (no pair of prime numbers with a gap of `g`)
In C: return [0, 0]
In C++, Lua, COBOL: return `{0, 0}`. 
In F#: return `[||]`. 
In Kotlin, Dart and Prolog: return `[]`.
In Pascal: return Type TGap (0, 0).
Examples:
- gap(2, 5, 7) --> [5, 7] or (5, 7) or {5, 7}

gap(2, 5, 5) --> nil. In C++ {0, 0}. In F# [||]. In Kotlin, Dart and Prolog return []`

gap(4, 130, 200) --> [163, 167] or (163, 167) or {163, 167}

([193, 197] is also such a 4-gap primes between 130 and 200 but it's not the first pair)

gap(6,100,110) --> nil or {0, 0} or ... : between 100 and 110 we have 101, 103, 107, 109 but 101-107is not a 6-gap because there is 103in between and 103-109is not a 6-gap because there is 107in between.

You can see more examples of return in Sample Tests.

Note for Go
For Go: nil slice is expected when there are no gap between m and n. Example: gap(11,30000,100000) --> nil

Ref
https://en.wikipedia.org/wiki/Prime_gap

Fundamentals



TEST CASES:
fn testing(g: i32, m: u64, n: u64, exp: Option<(u64, u64)>) -> () {
    assert_eq!(gap(g, m, n), exp)
}

#[test]
fn basics_gap() {
    testing(2,100,110, Some((101, 103)));
    testing(4,100,110, Some((103, 107)));
    testing(6,100,110, None);
    testing(8,300,400, Some((359, 367)));
    testing(10,300,400, Some((337, 347)));

    testing(4,30000,100000, Some((30109, 30113)));
    testing(6,30000,100000, Some((30091, 30097)));
    testing(8,30000,100000, Some((30161, 30169)));
    testing(11,30000,100000, None);
    testing(2,1000000,1100000, Some((1000037, 1000039)));
    //testing(2,10000000,11000000, Some((10000139, 10000141)));

    testing(4, 139148, 149148, Some((139297,139301)));
    testing(4, 428841, 438841, Some((429223,429227)));
    testing(4, 744472, 754472, Some((744637,744641)));
    testing(4, 254705, 264705, Some((254773,254777)));
    testing(4, 818533, 828533, Some((818683,818687)));
    testing(4, 83562, 93562, Some((83617,83621)));
    testing(4, 544742, 554742, Some((544879,544883)));
    testing(4, 980255, 990255, Some((980587,980591)));
    testing(4, 56673, 66673, Some((56779,56783)));
    testing(4, 203788, 213788, Some((203869,203873)));
    testing(4, 634196, 644196, Some((634237,634241)));
    testing(4, 543214, 553214, Some((543223,543227)));
    testing(4, 503503, 513503, Some((503959,503963)));
    testing(4, 882432, 892432, Some((882697,882701)));
    testing(4, 833642, 843642, Some((833713,833717)));
    testing(4, 7196, 17196, Some((7207,7211)));
    testing(7, 329939, 339939, None);
    testing(7, 552052, 562052, None);
    testing(4, 545574, 555574, Some((545617,545621)));
    testing(4, 205139, 215139, Some((205417,205421)));
    testing(4, 730827, 740827, Some((730909,730913)));
    testing(4, 401944, 411944, Some((401953,401957)));
    testing(4, 775487, 785487, Some((775513,775517)));
    testing(4, 250204, 260204, Some((250837,250841)));
    testing(4, 636819, 646819, Some((636997,637001)));
    testing(4, 973874, 983874, Some((973897,973901)));
    testing(4, 882906, 892906, Some((883117,883121)));
    testing(4, 695475, 705475, Some((695599,695603)));
    testing(4, 10127, 20127, Some((10159,10163)));
    testing(4, 975271, 985271, Some((975277,975281)));
    testing(4, 532641, 542641, Some((532687,532691)));
    testing(4, 747299, 757299, Some((747319,747323)));
    testing(4, 903287, 913287, Some((903403,903407)));
    testing(4, 774871, 784871, Some((775087,775091)));
    testing(4, 863323, 873323, Some((863689,863693)));
    testing(4, 832773, 842773, Some((832837,832841)));
    testing(4, 550315, 560315, Some((550657,550661)));
    testing(4, 629874, 639874, Some((629899,629903)));
    testing(4, 338479, 348479, Some((338563,338567)));
    testing(4, 321022, 331022, Some((321073,321077)));
    testing(4, 695218, 705218, Some((695239,695243)));
    testing(4, 377640, 387640, Some((377827,377831)));

    testing(6, 970106, 980106, Some((970231,970237)));
    testing(6, 787962, 797962, Some((787993,787999)));
    testing(6, 170144, 180144, Some((170207,170213)));
    testing(6, 125099, 135099, Some((125101,125107)));
    testing(6, 593208, 603208, Some((593381,593387)));
    testing(6, 156805, 166805, Some((156817,156823)));
    testing(6, 438301, 448301, Some((438527,438533)));
    testing(6, 194754, 204754, Some((194813,194819)));
    testing(6, 967787, 977787, Some((967931,967937)));
    testing(6, 593670, 603670, Some((593777,593783)));
    testing(6, 373344, 383344, Some((373447,373453)));
    testing(6, 465686, 475686, Some((465887,465893)));
    testing(6, 702185, 712185, Some((702193,702199)));
    testing(6, 619452, 629452, Some((619471,619477)));
    testing(6, 263490, 273490, Some((263513,263519)));
    testing(6, 408954, 418954, Some((409021,409027)));
    testing(6, 332120, 342120, Some((332303,332309)));
    testing(6, 861070, 871070, Some((861083,861089)));
    testing(6, 123541, 133541, Some((123631,123637)));
    testing(6, 201762, 211762, Some((201781,201787)));
    testing(6, 415728, 425728, Some((415873,415879)));
    testing(6, 372948, 382948, Some((372973,372979)));
    testing(6, 274394, 284394, Some((274661,274667)));
    testing(6, 586857, 596857, Some((586897,586903)));
    testing(6, 346381, 356381, Some((346433,346439)));
    testing(6, 852332, 862332, Some((852457,852463)));
    testing(6, 934051, 944051, Some((934051,934057)));
    testing(6, 27442, 37442, Some((27481,27487)));
    testing(7, 698221, 708221, None);
    testing(7, 759952, 769952, None);
    testing(7, 778165, 788165, None);
    testing(6, 354287, 364287, Some((354301,354307)));
    testing(6, 345731, 355731, Some((345733,345739)));
    testing(6, 646405, 656405, Some((646537,646543)));
    testing(6, 416886, 426886, Some((416957,416963)));
    testing(6, 470905, 480905, Some((470927,470933)));
    testing(6, 102424, 112424, Some((102533,102539)));
    testing(6, 318008, 328008, Some((318173,318179)));
    testing(6, 293439, 303439, Some((293453,293459)));
    testing(6, 554441, 564441, Some((554447,554453)));
    testing(6, 372746, 382746, Some((372763,372769)));
    testing(6, 1595, 11595, Some((1601,1607)));
    testing(6, 889320, 899320, Some((889367,889373)));

    testing(8, 93488, 103488, Some((93629,93637)));
    testing(8, 827337, 837337, Some((827591,827599)));
    testing(8, 830678, 840678, Some((830891,830899)));
    testing(8, 729292, 739292, Some((729293,729301)));
    testing(8, 436979, 446979, Some((437141,437149)));

}
