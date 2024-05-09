# Smart Contract-Driven Study Scholarships

This project implements a decentralized scholarship platform utilizing smart contracts built with the Soroban SDK on the Stellar Network.

## Key Features

* **Scholarship Creation:** Donors can easily establish scholarships with customizable eligibility criteria (e.g., GPA, major, financial need).
* **Student Applications:**  Students submit applications containing relevant data that is matched against scholarship requirements.
* **Automated Eligibility:** Smart contracts evaluate applications against scholarship criteria, ensuring fair and transparent selection.
* **Scholarship Disbursement:**  Eligible students automatically receive scholarship funds, reducing administrative overhead.

## Workflow

**1. Scholarship Creation**
   * A donor calls the `create_scholarship` function on the smart contract.
   * They specify the scholarship amount, eligibility criteria, and transfer the corresponding funds to the contract (if using cryptocurrency).

   [Visio Diagram: donor -> contract (create_scholarship)]

**2. Student Application**
   *  Students submit an application with the `apply` function.
   *  They provide the scholarship ID and data relevant to the eligibility criteria.

   [Visio Diagram: student -> contract (apply)]

**3. Automated Evaluation and Disbursement**
   * The `process_applications` function (ideally on a scheduled basis) iterates through scholarships and applications.
   *  Eligibility logic (in the `is_eligible` function) compares application data against scholarship criteria.
   *  Eligible students automatically receive awarded funds via a token contract interaction.

   [Visio Diagram: contract (process_applications) -> token contract (optional)]

## Project Structure

* `src/lib.rs`: Contains the core Soroban smart contract code.
* `Cargo.toml`: Project dependencies including the Soroban SDK.

## Getting Started

1. **Prerequisites:**
   * Rust development environment
   * Soroban SDK
   * Access to a Soroban network (local sandbox for testing or a live network)
2. **Clone the repository:** `git clone https://github.com/<your_username>/study-scholarships`
3. **Build the contract:** `cargo build --release`
4. **Deploy to the Soroban network:** (instructions dependent on your Soroban environment)

## Considerations

* **Eligibility Verification:** Explore potential integrations with trusted external data sources for robust verification.
* **Token Contract:** If using cryptocurrency for funds, you'll need to create or integrate with an existing token contract.

## Contributing 
We welcome contributions! Please refer to the contributing guidelines.
