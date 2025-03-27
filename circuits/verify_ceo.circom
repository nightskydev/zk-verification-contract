pragma circom 2.0.0;
include "../node_modules/circomlib/circuits/poseidon.circom";

template PersonalInfo() {
  // Hash the private inputs using Poseidon
  component nameHasher = Poseidon(15);
  component roleHasher = Poseidon(15);
  component birthdayHasher = Poseidon(15);
  component companyNameHasher = Poseidon(15);

  signal input name[15];          // Private input: name
  signal input role[15];          // Private input: role
  signal input birthday[15];      // Private input: birthday
  signal input companyName[15];   // Private input: company name

  nameHasher.inputs <== name;
  roleHasher.inputs <== role;
  birthdayHasher.inputs <== birthday;
  companyNameHasher.inputs <== companyName;

  signal output name_out;
  signal output role_out;
  signal output birthday_out;
  signal output companyName_out;

  name_out <== nameHasher.out;
  role_out <== roleHasher.out;
  birthday_out <== birthdayHasher.out;
  companyName_out <== companyNameHasher.out;

  // Enforce that the hashed private inputs match the constant expected hashes
  4203130618016961831408770638653325366880478848856764494148034853759773445968 === name_out;
  4203130618016961831408770638653325366880478848856764494148034853759773445968 === role_out;
  4203130618016961831408770638653325366880478848856764494148034853759773445968 === birthday_out;
  4203130618016961831408770638653325366880478848856764494148034853759773445968 === companyName_out;
}

component main = PersonalInfo();