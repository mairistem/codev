# Loans Specification

## Purpose

How the library records the books its members borrow, and how the
librarians see the loans that are late.

## Requirements

### Requirement: A loan is due three weeks after it starts

The system SHALL record each loan with its book, its member and its start
date, and SHALL set its due date twenty-one days after the start.

#### Scenario: A book borrowed on March 1st

- **GIVEN** a member borrowing a book on 2026-03-01
- **WHEN** the loan is recorded
- **THEN** its due date is 2026-03-22

### Requirement: The librarians see the overdue loans

The page `/loans/overdue` SHALL list every loan not returned whose due
date has passed, the most overdue first, with the book's title, the
member's name and the number of days late.

#### Scenario: Two late loans

- **GIVEN** a loan five days late and a loan two days late
- **WHEN** a librarian opens `/loans/overdue`
- **THEN** the page lists the five-day loan first, then the other
