#include <z3++.h>
#include <format>
#include <iostream>
#include <fstream>
#include <string> 
#include <filesystem>


void addIteration(
	int iterCount,
	z3::solver& s,
	z3::context& ctx,
	z3::expr_vector& aIterations,
	z3::expr_vector& bIterations
) {
	// Make A iteration variable
	z3::expr ai = ctx.int_const(("a"+ std::to_string(iterCount)).c_str());
	z3::expr prevA{aIterations[aIterations.size()-1]};
	aIterations.push_back(ai);

	// Make B iteration variable
	z3::expr bi = ctx.int_const(("b"+ std::to_string(iterCount)).c_str());
	z3::expr prevB{bIterations[bIterations.size()-1]};
	bIterations.push_back(bi);

	// Make Formula for if __nondet__() returns true
	z3::expr iterationTrue = bi == prevB + 3 && ai == prevA + 2*bi;

	// Make Formula for if __nondet__() returns false
	z3::expr iterationFalse = bi == prevB + prevA && ai == prevA + 5;

	// Make Formula for both cases, Since its nondeterministic either paths need to be correct
	s.add(z3::implies(prevA < 180, iterationTrue || iterationFalse));
	// To make sure sure that that bounded iterations wont faulter the equivalence to the code 
	// we interpret the code as
	// while( in bound) if (a < 180) /* original code */ else { a = a; b = b};
	// This would carry the b and a values to the next
	// For example if we bound to 15 iterations but only need 10(e.g. since a is then 181)
	// the last 5 iterations would then be the same to make sure that the value of b 
	// (which wouldnt change after iteration 10) is passed to iteration 15
	s.add(z3::implies(prevA >= 180, ai == prevA && bi == prevB));

};
void writeToFile(z3::solver& s, std::string fileName)
{
	std::ofstream file{ fileName };

	// If we couldn't open the output file stream for writing
	if (!file)
	{
		// Print an error and exit
		std::cerr << "file could not be opened for writing.\n";
		std::exit(1);
	}	
	file << s.get_model();
}
void makeFormulaWithinBound(
	int maxBound,
	z3::expr k,
	z3::solver& s,
	z3::context& ctx,
	z3::expr_vector& aIterations,
	z3::expr_vector& bIterations
)
{
	for (int bound{1}; bound <= maxBound;bound++) 
	{
		addIteration(bound, s,ctx, aIterations, bIterations);
	}
	z3::expr lastB{bIterations[bIterations.size()-1]};
	s.add((lastB == 190 + k));
}

int main(int argc, char* argv[])
{
	// create folder to output the result of the SMT
	std::filesystem::create_directories("./output");

    z3::context ctx;
    z3::solver s(ctx);

	int maxBound{40}; // a slows the slowest if the else branch is only taken
	

	z3::expr_vector aIterations{ctx};
	z3::expr_vector bIterations{ctx};
	z3::expr a0 = ctx.int_const("a0");
	s.add(a0 == 1);

	z3::expr b0 = ctx.int_const("b0");
	s.add(b0 == 1);
	z3::expr k = ctx.int_const("k");

	aIterations.push_back(a0);
	bIterations.push_back(b0);
	makeFormulaWithinBound(maxBound, k,s,ctx,aIterations, bIterations);

	for (int i{0z}; i <= 10; i++) 
	{
		std::cout << "Checking for k = " << i << ": ";
		std::string fileName{"output/k"+std::to_string(i)+".txt"};
		s.push();
		s.add(k == i);
		if ( s.check() == z3::sat) {
			std::cout << "SAT: Program can crash" << std::endl;
			writeToFile(s,fileName);
		} else {
			std::cout << "UNSAT: Program can not crash" << std::endl;
		}
		s.pop();
	}
    return 0;
}


/*
Consider the following program written in C. Here, nondet () is a function that non-
deterministically returns an arbitrary Boolean value.
int main() {
	int a = 1;
	int b = 1;
	while (a < 180) {
		if (__nondet__()) {
			b = b + 3;
			a = a + 2*b;
		} else {
			b = b + a;
			a = a + 5;
		}
	}
	if (b == 190 + k) {
		assert(false);
	}
}
Show for each value of k ∈ {0, . . . , 10} whether the program is safe or can crash (i.e. reach
the assert statement). If a crash is possible, provide one complete run leading to that crash.


We translate this for UNSAT to
a_0 = 1 /\ b_0 = 1
(/\_1<i<q 
	(a_i-1 < 180 => 
		(b_i = b_i-1 + 3 /\ a_i = a_i-1 + 2 * b_i) \/
		(b_i = b_i-1 + a_i-1 /\ a_i = a_i-1 + 5)
	) /\
	(a_i-1 >= 180 => (
		(b_i = b_i-1 /\ a_i = a_i-1)
	)
) /\
(b_q = 190 + k)
*/
