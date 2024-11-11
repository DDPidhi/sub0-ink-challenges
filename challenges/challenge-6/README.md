# ✒️ Challenge 6: Ideate and design a multichain, permissionless solution for Polkadot

- **Difficulty:** Easy
- **Submission Criteria:** A PDF containing
    - A write-up of their proposal
    - What their idea solves, how it works, and team members (if applicable)
- **Submission Guidelines:** Verify with R0GUE DevRel

---

**Team: Feooh.com**

**Proposal for Blockchain-Based Developer Challenge Platform**

**Overview**

In the current landscape of developer events, challenges often require participants to submit their solutions for validation and scoring within a specific timeframe. This process typically involves manual handling of submissions, verification against test cases, and announcement of results. We propose a blockchain-based solution leveraging Polkadot parachains to automate and streamline this workflow, enhancing transparency, security, and efficiency.

**Objective**

Our goal is to create a decentralized platform that automates the end-to-end process of developer challenge submissions. This platform will:

* Ensure transparent submission tracking and validation.
* Automate the execution of test cases to validate submissions.
* Maintain immutable records of successful submissions.
* Announce winners automatically based on predefined criteria, ensuring fairness.

**Key Features**

1. **Decentralized Submission System**: Participants will submit their solutions directly onto the blockchain, ensuring that submissions are time-stamped and tamper-proof.
2. **Automated Validation Module**: Each submission will be evaluated automatically against a set of test cases, leveraging smart contract logic to determine validity.
3. **Immutable Leaderboard**: A real-time leaderboard will be maintained on-chain, showcasing participants’ standings based on the number of successfully validated submissions.
4. **Transparent Winner Selection**: At the end of the challenge period, the platform will execute logic to determine the participant with the highest number of successful submissions and announce them as the winner. In case of a tie, the solution will handle the tie using one of the following organizer-selected options:
  * **Community Vote**: Initiate a vote where the community decides whose challenge solutions are the best.
  * **First Submission Advantage**: Declare the winner based on who submitted their solution first. If a tie still exists, a community vote will be triggered as a fallback.

**Technical Architecture**

* **Polkadot Parachains**: The core of the platform will utilize Polkadot parachains to ensure scalability, interoperability, and security. By building on Polkadot, the solution will leverage the robust infrastructure for seamless cross-chain communication and transaction finality.
* **Smart Contracts**: Challenge submission and validation will be governed by smart contracts written in ink\!, a language designed for Substrate, enabling secure and efficient contract execution.
* **On-Chain Storage**: Submissions, results, and leaderboard data will be stored on-chain to maintain integrity and transparency.
* **Decentralized Execution Environment**: Validation of code submissions will be conducted in a decentralized manner, ensuring unbiased assessment.
* **IPFS Integration**: The participant will submit their code, which will be uploaded to IPFS. The hash of the IPFS upload will be stored on-chain. A smart contract will retrieve the code from IPFS, run tests against it on a cloud server (similar to how tests are run via GitHub Actions), and grade the code based on various metrics. The remaining steps, such as increasing the count of submissions per participant and declaring the winner, will follow.

**Workflow**

1. **Submission Phase**:
  * Developers submit their solutions to the blockchain before the deadline.
  * Each submission is time-stamped and logged for transparency.
2. **Validation Phase**:
  * Submissions are automatically run against a set of predefined test cases.
  * Smart contracts validate the submissions and mark them as successful or unsuccessful.
  * The participant's code is uploaded to IPFS, and the resulting hash is stored on-chain.
  * The smart contract retrieves the code from IPFS and runs tests on a cloud server, grading the code based on specific metrics.
3. **Result Announcement**:
  * The leaderboard is updated in real-time.
  * At the challenge deadline, the platform announces the participant with the most successful submissions as the winner. If there is a tie, the organizer’s chosen tie-breaking mechanism will be executed.

**Benefits**

* **Transparency**: All transactions and records are publicly accessible, ensuring trust in the submission and validation process.
* **Automation**: Reduces manual intervention, enhancing operational efficiency.
* **Security**: Submissions are secure and tamper-proof due to blockchain's immutable nature.
* **Fairness**: Eliminates biases, as validation is performed via automated, pre-set logic.

**Implementation Plan**

1. **Phase 1 – Development & Testing**:
  * Develop smart contracts for submission and validation logic.
  * Build the parachain infrastructure and deploy test cases.
2. **Phase 2 – Pilot Launch**:
  * Run a pilot with selected developer communities to test and refine the system.
3. **Phase 3 – Full Deployment**:
  * Scale the solution for broader use across global developer events.

**Conclusion**

By building this solution on Polkadot parachains, we can automate developer challenge submissions, validation, and winner selection in a decentralized, transparent, and secure manner. This will not only streamline the process but also foster trust and participation in developer events.


