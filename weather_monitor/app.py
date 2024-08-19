from flask import Flask, render_template, redirect, url_for, request, flash, session
from flask_login import LoginManager, UserMixin, login_user, login_required, logout_user, current_user
import requests
import json

app = Flask(__name__)
app.secret_key = 'your_secret_key'

# Flask-Login setup
login_manager = LoginManager()
login_manager.init_app(app)
login_manager.login_view = 'login'

# Dummy user data
with open('users.json', 'r') as f:
    users = json.load(f)


class User(UserMixin):
    def __init__(self, id):
        self.id = id


@login_manager.user_loader
def load_user(user_id):
    if user_id in users:
        return User(user_id)
    return None


@app.route('/')
@login_required
def home():
    city = request.args.get('city', "Lucknow")

    # Fetch the weather and air quality data from the API
    data = fetch_data_from_weather_api(city)

    # Parse the weather and air quality data
    weather_data = parse_weather_data(data)
    air_quality_data = parse_air_quality(data)

    return render_template('home.html', weather=weather_data, air_quality=air_quality_data, city=city)


@app.route('/login', methods=['GET', 'POST'])
def login():
    if request.method == 'POST':
        username = request.form['username']
        password = request.form['password']
        if username in users and users[username]['password'] == password:
            user = User(username)
            login_user(user)
            return redirect(url_for('home'))
        else:
            flash('Invalid username or password')
    return render_template('login.html')


@app.route('/logout')
@login_required
def logout():
    logout_user()
    return redirect(url_for('login'))


def fetch_data_from_weather_api(city):
    api_key = 'API_KEY_HERE'  # Replace with your actual API key
    response = requests.get(f'http://api.weatherapi.com/v1/current.json?key={api_key}&q={city}&aqi=yes')
    data = response.json()
    return data


def parse_weather_data(data):
    return {
        'city': data['location']['name'],
        'temperature': data['current']['temp_c'],
        'humidity': data['current']['humidity'],
        'wind_speed': data['current']['wind_kph']
    }


def parse_air_quality(data):
    return {
        'co': data['current']['air_quality']['co'],  # Carbon monoxide (CO) in µg/m³
        'no2': data['current']['air_quality']['no2'],  # Nitrogen dioxide (NO2) in µg/m³
        'o3': data['current']['air_quality']['o3'],  # Ozone (O3) in µg/m³
        'so2': data['current']['air_quality']['so2'],  # Sulfur dioxide (SO2) in µg/m³
        'pm2_5': data['current']['air_quality']['pm2_5'],  # PM2.5 in µg/m³
        'pm10': data['current']['air_quality']['pm10'],  # PM10 in µg/m³
        'us_epa_index': data['current']['air_quality']['us-epa-index'],  # US EPA AQI index
        'gb_defra_index': data['current']['air_quality']['gb-defra-index']  # UK Defra AQI index
    }


if __name__ == '__main__':
    app.run(debug=True)
