class CryptoArbScanner {
    constructor() {
        this.exchanges = [];
        this.autoScanInterval = null;
        this.init();
    }

    async init() {
        await this.loadExchanges();
        this.bindEvents();
    }

    async loadExchanges() {
        try {
            const response = await fetch('/api/exchanges');
            this.exchanges = await response.json();
            
            const exchangeSelect = document.getElementById('exchange');
            exchangeSelect.innerHTML = '';
            
            this.exchanges.forEach(exchange => {
                if (exchange.enabled) {
                    const option = document.createElement('option');
                    option.value = exchange.id;
                    option.textContent = exchange.name;
                    exchangeSelect.appendChild(option);
                }
            });
            
            document.getElementById('scanBtn').disabled = false;
            document.getElementById('autoScanBtn').disabled = false;
        } catch (error) {
            this.showError('Failed to load exchanges: ' + error.message);
        }
    }

    bindEvents() {
        document.getElementById('scanBtn').addEventListener('click', () => this.scan());
        document.getElementById('autoScanBtn').addEventListener('click', () => this.toggleAutoScan());
    }

    async scan() {
        const exchangeId = document.getElementById('exchange').value;
        const minProfit = parseFloat(document.getElementById('minProfit').value) || 0.1;

        if (!exchangeId) {
            this.showError('Please select an exchange');
            return;
        }

        this.showLoading(true);
        this.hideError();

        try {
            const response = await fetch('/api/scan', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    exchange_id: parseInt(exchangeId),
                    min_profit: minProfit
                })
            });

            const data = await response.json();

            if (response.ok) {
                this.displayResults(data);
            } else {
                this.showError(data.error || 'Scan failed');
            }
        } catch (error) {
            this.showError('Network error: ' + error.message);
        } finally {
            this.showLoading(false);
        }
    }

    toggleAutoScan() {
        const btn = document.getElementById('autoScanBtn');
        
        if (this.autoScanInterval) {
            clearInterval(this.autoScanInterval);
            this.autoScanInterval = null;
            btn.textContent = 'Auto Scan (10s)';  // Updated text
            btn.classList.remove('btn-danger');
            btn.classList.add('btn-secondary');
        } else {
            this.autoScanInterval = setInterval(() => this.scan(), 10000);  // 10 seconds
            btn.textContent = 'Stop Auto Scan';
            btn.classList.remove('btn-secondary');
            btn.classList.add('btn-danger');
        }
    }

    displayResults(data) {
        // Update stats
        document.getElementById('totalPairs').textContent = data.total_pairs;
        document.getElementById('totalOpportunities').textContent = data.opportunities.length;
        document.getElementById('scanTime').textContent = data.scan_time_ms + 'ms';

        // Display opportunities in table format
        const opportunitiesList = document.getElementById('opportunitiesList');
        opportunitiesList.innerHTML = '';

        if (data.opportunities.length === 0) {
            opportunitiesList.innerHTML = '<p class="no-opportunities">No arbitrage opportunities found above the minimum profit threshold.</p>';
            return;
        }

        // Create table
        const table = document.createElement('table');
        table.className = 'opportunities-table';
        
        table.innerHTML = `
            <thead>
                <tr>
                    <th>Path</th>
                    <th>Pairs</th>
                    <th>Gross Profit %</th>
                    <th>Fees %</th>
                    <th>Net Profit %</th>
                </tr>
            </thead>
            <tbody>
                ${data.opportunities.map(opportunity => `
                    <tr class="${this.getProfitRowClass(opportunity.net_profit_percentage)}">
                        <td class="path-cell">${opportunity.path}</td>
                        <td>${opportunity.pairs}</td>
                        <td class="${this.getProfitClass(opportunity.gross_profit_percentage)}">${opportunity.gross_profit_percentage.toFixed(4)}%</td>
                        <td>${opportunity.estimated_fees.toFixed(4)}%</td>
                        <td class="${this.getProfitClass(opportunity.net_profit_percentage)}">${opportunity.net_profit_percentage.toFixed(4)}%</td>
                    </tr>
                `).join('')}
            </tbody>
        `;

        opportunitiesList.appendChild(table);
    }

    getProfitRowClass(netProfit) {
        if (netProfit >= 2.0) return 'high-profit';
        if (netProfit >= 1.0) return 'medium-profit';
        return 'low-profit';
    }

    getProfitClass(netProfit) {
        if (netProfit >= 2.0) return 'profit-high';
        if (netProfit >= 1.0) return 'profit-medium';
        return 'profit-low';
    }

    showLoading(show) {
        document.getElementById('loading').style.display = show ? 'block' : 'none';
    }

    showError(message) {
        const errorDiv = document.getElementById('error');
        errorDiv.textContent = message;
        errorDiv.style.display = 'block';
        setTimeout(() => this.hideError(), 5000);
    }

    hideError() {
        document.getElementById('error').style.display = 'none';
    }
}

// Initialize the app when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    new CryptoArbScanner();
});