import React, { useState } from 'react';
import { checkGitHub, checkTwitter } from '../utils/inferoApi';

function AddressInput() {
  const [address, setAddress] = useState('');
  const [githubResult, setGitHubResult] = useState('');
  const [twitterResult, setTwitterResult] = useState('');

  const handleCheck = async () => {
    const github = await checkGitHub(address);
    const twitter = await checkTwitter(address);
    setGitHubResult(github);
    setTwitterResult(twitter);
  };

  return (
    <div>
      <input
        type="text"
        value={address}
        placeholder="Enter GitHub or Twitter handle"
        onChange={(e) => setAddress(e.target.value)}
      />
      <button onClick={handleCheck}>Check</button>
      <div>
        <p><strong>GitHub:</strong> {githubResult}</p>
        <p><strong>Twitter:</strong> {twitterResult}</p>
      </div>
    </div>
  );
}

export default AddressInput;
