Feature: State

  Scenario: fetch state
    Given validator 1 is joined
      And validator 2 is joined
     When requesting contract state
     Then the response code is 200
      And validator 1 is listed as a validator
      And validator 2 is listed as a validator

  Scenario: fetch state at certain block height
    Given validator 1 is joined
      And validator 2 is joined
     When requesting contract state one block earlier
     Then the response code is 200
      And validator 1 is listed as a validator
      And validator 2 is not listed as a validator
