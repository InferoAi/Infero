export async function checkGitHub(repo) {
  try {
    const response = await fetch(`http://localhost:8080/check/github/${repo}`);
    return await response.text();
  } catch (err) {
    return "Error reaching backend.";
  }
}

export async function checkTwitter(username) {
  try {
    const response = await fetch(`http://localhost:8080/check/twitter/${username}`);
    return await response.text();
  } catch (err) {
    return "Error reaching backend.";
  }
}
