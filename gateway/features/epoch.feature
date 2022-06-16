Feature: Epoch

  Scenario: update epoch
    Given validator 1 is joined
     When requesting to update epoch
     Then the response code is 200
      And the response data contains valid tx
