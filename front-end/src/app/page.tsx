'use client'
import { useState } from 'react';
import axios from 'axios';

export default function Home() {
  const [domain, setDomain] = useState('');
  const [loading, setLoading] = useState(false);
  const [certificateData, setCertificateData] = useState<any>(null);
  const [error, setError] = useState('');

  const handleSubmit = async () => {
    setLoading(true);
    setError('');
    setCertificateData(null);

    try {
      const response = await axios.post('http://127.0.0.1:8080/validate_certificate', { domain });
      setCertificateData(response.data);
    } catch (err) {
      setError('Failed to fetch certificate data. Please check the domain and try again.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="container">
      <h1>SSL Certificate Checker</h1>
      <div className="input-section">
        <input
          type="text"
          placeholder="Enter domain name"
          value={domain}
          onChange={(e) => setDomain(e.target.value)}
        />
        <button onClick={handleSubmit} disabled={loading || !domain}>
          {loading ? 'Checking...' : 'Check SSL'}
        </button>
      </div>

      {error && <div className="error-message">{error}</div>}

      {certificateData && (
        <div className="results-section">
          <h2>Certificate Details</h2>
          <p><strong>Validity Status:</strong> {certificateData.validity_status ? 'Valid' : 'Invalid'}</p>
          <p><strong>Expiration Date:</strong> {certificateData.expiration_date}</p>
          <p><strong>Issuer Details:</strong> {certificateData.issuer_details}</p>
          <p><strong>Subject Details:</strong> {certificateData.subject_details}</p>
          <p><strong>Valid for Domain:</strong> {certificateData.is_valid_for_domain ? 'Yes' : 'No'}</p>
          <p><strong>CA Validity Check:</strong> {certificateData.ca_validity_check ? 'Valid CA' : 'Not a CA'}</p>
          <p><strong>Not Self-Signed:</strong> {certificateData.is_not_self_signed ? 'Yes' : 'No'}</p>
          <p><strong>CRL/OCSP Status:</strong> {certificateData.revocation_status}</p>
        </div>
      )}

      <style jsx>{`
        .container {
          max-width: 600px;
          margin: 0 auto;
          padding: 20px;
          text-align: center;
        }
        .input-section {
          margin-bottom: 20px;
        }
        input {
          padding: 10px;
          width: 300px;
          margin-right: 10px;
        }
        button {
          padding: 10px 20px;
          cursor: pointer;
        }
        .results-section {
          margin-top: 20px;
          text-align: left;
        }
        .error-message {
          color: red;
          margin-top: 10px;
        }
      `}</style>
    </div>
  );
}
