Feature: Slashing

  Scenario: validator proposes slashing
    Given the validator is joined
     When the validator proposes slashing because of a missing transaction
     Then the response code is 200
      And the proposal is recorded in the contract state

  Scenario: validator votes for slashing
    Given validator 1 is joined
      And validator 2 is joined
      And validator 2 has proposed for slashing because of a missing transaction
     When the validator votes "for" slashing
     Then the vote "for" is recorded in the contract state

  Scenario: validator votes against slashing
    Given validator 1 is joined
      And validator 2 is joined
      And validator 2 has proposed for slashing because of a missing transaction
     When the validator votes "against" slashing
     Then the vote "against" is recorded in the contract state

  Scenario: invalid proposal data yields client error
     When posting invalid proposal data
     Then the response code is 400

  Scenario: invalid voting data yields client error
     When posting invalid voting data
     Then the response code is 400

  Scenario: fetch validators contract state
     When requesting contract state
     Then the response code is 200
      And response body is valid JSON for validators contract state
