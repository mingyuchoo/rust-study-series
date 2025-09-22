import { useState, useEffect } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

interface ApiData {
  message: string;
  items: Array<{
    id: number;
    name: string;
    description: string;
  }>;
}

function App() {
  const [count, setCount] = useState(0)
  const [apiData, setApiData] = useState<ApiData | null>(null)
  const [loading, setLoading] = useState(false)

  const fetchData = async () => {
    setLoading(true)
    try {
      const response = await fetch('/api/data')
      const data = await response.json()
      setApiData(data)
    } catch (error) {
      console.error('Failed to fetch data:', error)
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    fetchData()
  }, [])

  return (
    <>
      <div>
        <a href="https://vite.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>React + ASP.NET Core</h1>
      
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>

      <div className="card">
        <h2>API Data</h2>
        {loading ? (
          <p>Loading...</p>
        ) : apiData ? (
          <div>
            <p><strong>{apiData.message}</strong></p>
            <ul>
              {apiData.items.map(item => (
                <li key={item.id}>
                  <strong>{item.name}</strong>: {item.description}
                </li>
              ))}
            </ul>
            <button onClick={fetchData}>Refresh Data</button>
          </div>
        ) : (
          <p>No data available</p>
        )}
      </div>

      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  )
}

export default App
