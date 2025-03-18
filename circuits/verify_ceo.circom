pragma circom 2.0.3;

include "../node_modules/circomlib/circuits/sha256/sha256.circom";

/*
    Circuit to verify CEO's identity using hashed personal information.
    Public Inputs:
        - nameHash: Hash of the CEO's name
        - roleHash: Hash of the CEO's role
        - companyNameHash: Hash of the company name
        - registrationIdHash: Hash of the company registration ID
    Private Inputs:
        - name: CEO's name
        - role: CEO's role
        - companyName: Company name
        - registrationId: Company registration ID
*/

template VerifyCEO() {
    // Public inputs
    signal input nameHash[32];          // Hash of the CEO's name (32 bytes)
    signal input roleHash[32];          // Hash of the CEO's role (32 bytes)
    signal input companyNameHash[32];   // Hash of the company name (32 bytes)
    signal input registrationIdHash[32]; // Hash of the company registration ID (32 bytes)

    // Private inputs
    signal input name[32];              // CEO's name as an array of bytes
    signal input role[32];              // CEO's role as an array of bytes
    signal input companyName[32];       // Company name as an array of bytes
    signal input registrationId[32];    // Company registration ID as an array of bytes

    // Hash the private inputs
    component nameHasher = Sha256(32);
    component roleHasher = Sha256(32);
    component companyNameHasher = Sha256(32);
    component registrationIdHasher = Sha256(32);

    // Ensure inputs are padded to 32 bytes
    for (var i = 0; i < 32; i++) {
        nameHasher.in[i] <== name[i];
        roleHasher.in[i] <== role[i];
        companyNameHasher.in[i] <== companyName[i];
        registrationIdHasher.in[i] <== registrationId[i];
    }

    // Ensure the hashes match the public inputs
    for (var i = 0; i < 32; i++) {
        nameHasher.out[i] === nameHash[i];
        roleHasher.out[i] === roleHash[i];
        companyNameHasher.out[i] === companyNameHash[i];
        registrationIdHasher.out[i] === registrationIdHash[i];
    }
}

component main = VerifyCEO();