Feature: Requests and reports all API end points. (GET /query)
    Scenario Outline: Get server time information.
        Given user set base url for retriving the server time
        When user retrievs the server time details
        Then the response should be successful

    Scenario Outline: Get trading pair information.
        Given user set base url for trading pair
        When user retrievs the trading pair '<pair>' information
        Then the response should be successful
        Examples:
            | pair    |
            | XBTUSD |

    Scenario: Retrieve all open orders from an account
        Given user set base url for an account
        When user request the open orders
