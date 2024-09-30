# SSL Certificate Validator

This project provides an SSL Certificate Validator, which checks the validity, issuer, subject details, and revocation status of SSL certificates for a given domain. The project includes both backend and frontend components:

- **Backend**: Built using Rust and Actix Web.
- **Frontend**: Built using Next.js and React.

## Setup Instructions

### Prerequisites

- Rust and Cargo installed.
- Node.js and npm (or Yarn) installed.
- OpenSSL development libraries installed.

### Backend Setup (Rust + Actix Web)

1. **Navigate to the Backend Directory**:
   ```sh
   cd backend
   ```
   
2. **Install Dependencies**:
   Ensure that `openssl` is installed. You can use a package manager like `apt` (Linux) or `brew` (macOS):
   ```sh
   # For Ubuntu
   sudo apt-get install libssl-dev

   # For macOS
   brew install openssl
   ```

3. **Build and Run the Backend Server**:
   ```sh
   cargo run
   ```
   The backend server will start on `http://127.0.0.1:8080`.

### Frontend Setup (Next.js)

1. **Navigate to the Frontend Directory**:
   ```sh
   cd ../frontend
   ```

2. **Install Dependencies**:
   ```sh
   npm install
   ```
   or if you use Yarn:
   ```sh
   yarn install
   ```

3. **Run the Frontend**:
   ```sh
   npm run dev
   ```
   The frontend will start on `http://localhost:3000`.

### Testing the Application

1. **Access the Frontend**:
   Open your browser and go to `http://localhost:3000`.

2. **Enter a Domain**:
   Enter a domain (e.g., `example.com`) in the input field and click on "Check SSL" to validate the SSL certificate.

## Technology Choices

### Backend: Rust and Actix Web

- **Rust**: Rust was chosen for its high performance and safety, particularly around memory management, which makes it well-suited for handling low-level networking tasks like SSL certificate validation.
- **Actix Web**: Actix Web is a fast and flexible framework for building web applications in Rust. It supports asynchronous operations, making it an ideal choice for handling multiple requests in a performant manner.

### Frontend: Next.js

- **Next.js**: Next.js is a popular React framework with built-in support for server-side rendering (SSR), static site generation (SSG), and easy routing. Its flexibility and performance make it ideal for creating a fast and interactive frontend.
- **React**: React is a widely-used JavaScript library for building user interfaces, offering a rich ecosystem of components and libraries to accelerate development.

## Assumptions and Design Decisions

1. **SSL Certificate Verification**: 
   - The application assumes that certificates are issued by trusted Certificate Authorities (CAs). 
   - Certificate validation against OCSP/CRL is simplified and currently set as "Unknown" due to the complexity of implementing full OCSP/CRL checks.

2. **Domain Input**: 
   - The domain entered by the user is expected to be valid and resolvable. No advanced validation of domain syntax is implemented beyond basic checks.

3. **Security Considerations**:
   - Certificate verification is disabled in the current setup (i.e., `connector_builder.set_verify(openssl::ssl::SslVerifyMode::NONE)`), which is only for development purposes. In a production environment, this would need to be enabled for proper SSL security.

## Known Limitations and Areas for Improvement

1. **Certificate Revocation Checks**:
   - The current implementation does not fully check OCSP or CRL status due to its complexity. Future versions could improve on this by integrating more advanced OCSP/CRL checking mechanisms.

2. **Error Handling**:
   - Error handling in both backend and frontend is minimal. More comprehensive error messages and logging mechanisms could improve the user experience and debuggability.

3. **Frontend Validation**:
   - The frontend could benefit from additional validation on the input field to ensure the domain entered is properly formatted (e.g., using a regex for domain validation).

4. **Performance Optimization**:
   - For high-volume environments, the SSL certificate fetch operation could be optimized with a caching mechanism to reduce repeated lookups for the same domain.

## License

This project is open-source and available under the MIT License.
