pragma circom 2.1.0;

include "poseidon.circom";
include "comparators.circom";

template DeckCommitment(n) {
    signal input salt;
    signal input deck[n];
    signal output root;

    component h0 = Poseidon(2);
    h0.inputs[0] <== salt;
    h0.inputs[1] <== deck[0];

    component chain[n - 1];
    for (var i = 0; i < n - 1; i++) {
        chain[i] = Poseidon(2);
        if (i == 0) {
            chain[i].inputs[0] <== h0.out;
            chain[i].inputs[1] <== deck[1];
        } else {
            chain[i].inputs[0] <== chain[i - 1].out;
            chain[i].inputs[1] <== deck[i + 1];
        }
    }
    root <== chain[n - 2].out;
}

template CardInDeck(n) {
    signal input card;
    signal input deck[n];

    component eq[n];
    signal hits[n];
    for (var j = 0; j < n; j++) {
        eq[j] = IsEqual();
        eq[j].in[0] <== card;
        eq[j].in[1] <== deck[j];
        hits[j] <== eq[j].out;
    }

    signal total[n + 1];
    total[0] <== 0;
    for (var k = 0; k < n; k++) {
        total[k + 1] <== total[k] + hits[k];
    }

    component has = GreaterThan(16);
    has.in[0] <== total[n];
    has.in[1] <== 0;
    has.out === 1;
}

template CardRange(maxCard) {
    signal input card;

    component lo = GreaterEqThan(16);
    lo.in[0] <== card;
    lo.in[1] <== 1;
    lo.out === 1;

    component hi = LessEqThan(16);
    hi.in[0] <== card;
    hi.in[1] <== maxCard;
    hi.out === 1;
}

template HandUnique(handSize) {
    signal input hand[handSize];

    component eq0 = IsEqual();
    eq0.in[0] <== hand[0];
    eq0.in[1] <== hand[1];
    eq0.out === 0;

    component eq1 = IsEqual();
    eq1.in[0] <== hand[0];
    eq1.in[1] <== hand[2];
    eq1.out === 0;

    component eq2 = IsEqual();
    eq2.in[0] <== hand[0];
    eq2.in[1] <== hand[3];
    eq2.out === 0;

    component eq3 = IsEqual();
    eq3.in[0] <== hand[0];
    eq3.in[1] <== hand[4];
    eq3.out === 0;

    component eq4 = IsEqual();
    eq4.in[0] <== hand[1];
    eq4.in[1] <== hand[2];
    eq4.out === 0;

    component eq5 = IsEqual();
    eq5.in[0] <== hand[1];
    eq5.in[1] <== hand[3];
    eq5.out === 0;

    component eq6 = IsEqual();
    eq6.in[0] <== hand[1];
    eq6.in[1] <== hand[4];
    eq6.out === 0;

    component eq7 = IsEqual();
    eq7.in[0] <== hand[2];
    eq7.in[1] <== hand[3];
    eq7.out === 0;

    component eq8 = IsEqual();
    eq8.in[0] <== hand[2];
    eq8.in[1] <== hand[4];
    eq8.out === 0;

    component eq9 = IsEqual();
    eq9.in[0] <== hand[3];
    eq9.in[1] <== hand[4];
    eq9.out === 0;
}