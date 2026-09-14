# 2IMF25 Automated Reasoning

In this course, we study a number of techniques and algorithms, including
satisfiability for propositional (Boolean) logic, satisfiability modulo
theories (SMT), binary decision diagrams (BDDs), model checking, and
first-order logic. The course covers both the underlying algorithms and solving
real-world problems using the tools that implement those algorithms.

## Organizational Information

The practical assignment has to be executed by groups of three persons.
Registration in groups must be done via Canvas, at the latest on
**9 September 2026**. After that date, incomplete groups will be merged
and students who are not in a group will be considered as no longer taking
part in the course. 

The results of the assignment has to be described in a report that should be
submitted as PDF next to the source code of your implementation in a ZIP-file
via Ans. For the report the deadline is **16 October 2026**. Additional
information about the course can be found at the course page in Canvas.

## Report Requirements

The report must be written in $\LATEX$ and have less than 20 pages with a
standard page and font size, e.g., Springers `llncs` style or the `article`
class. Source code of your implementations should be avoided in the main
part of the report, presenting only small relevant excerpts if necessary for
illustration purposes. For all variables and formulas documentation is
required, explaining the approach and the overall structure. A generic approach
is preferred, since this may result in clearer descriptions, increasing the
confidence in the correctness of the results. Formulas of more than half a page
should not be contained in the report, instead the structure of the formula
should be given using abstract notation. From the output of the programs only
relevant parts should be contained in the report, and observations on
computation time should be reported. The answers on the problems should be
motivated, where relevant including tables or pictures is required. In summary,
each answer should at least contain:

- Description of the parameters and variables and their types.
- Formulas that capture the problem.
- Description of how the tool was used to obtain the desired answer.
- Visualization of the results (table, drawing or other).
- Discussion of runtime.

Every report should contain name, student number and email address of each of
the authors as well as the number of the group. Each author is considered to
be responsible for the full text and all results.

## Guidelines for grading

The assignment consists of seven tasks, the weight of each task is given in the
description. The grading will mainly take into account the following aspects:

- Clarity and conciseness of the formulas and accompanying descriptions.
- Correctness of the formulas and of the approach and the argumentation behind
  it.
- Correctness and completeness of the reported results.

If you do not manage to (completely) solve an exercise, clearly explain the
limitations of your solution or the obstacles you encountered. An example of
the desired style is provided on Canvas.

The calculation of the grade from the number of points scored is not fixed in
advance, so solving half the tasks perfectly does not necessarily mean you
obtain a 5.5.

## Tools and Libraries

Only the following tools or libraries should be used to solve the problems:

- [Z3](https://github.com/Z3Prover/z3) or [Yices](http://yices.csl.sri.com/)
  Both Z3 and Yices are programs for satisfiability modulo theories (SMT). They
  accept standard SMT format, in particular boolean SAT format. Z3 provides
  APIs for several popular programming languages, Yices only provides a C API.
- [OxiDD](https://oxidd.net)
  OxiDD is a modern BDD framework written in Rust, with available APIs for
  Python and C++. Please use the BDDManager (BDDs without complement edges) to
  solve the assignments. 
  
Each of the problems should be solved using one of these tools and assessed
with current versions linked above

## Use of AI

Use of (generative) AI is allowed for obtaining general knowledge, but not for directly solving
the given problems or translating code to formulas. Your report is required to include a
statement on whether you used AI, and, if so, how. The general guidelines from the TU/e
can be found [here](https://assets.w3.tue.nl/w/fileadmin/Education_Guide/Content/Programs/Testing%20and%20assessment/AI%20Rules_TUe.pdf).
