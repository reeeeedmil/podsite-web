# Webová aplikace na výpočet podsítí
## Obsah
1. Instalace pro hostování aplikace
2. Ovládání aplikace pomocí CLI
3. Frontend
4. Backend
    - Python
        - Django
        - Django Rest Framework
    - Rust
        - Maturin
        - PyO3

## 1. Instalace
### Požadavky pro frontend
- Instalace nodejs a npm
- poté pomocí npm nainstalovat potřebné balíčky
- Ve složce frontend zadejte příkaz "npm install"

### Požadavky pro backend
- Instalace Pythonu
- Instalace Rustu
- Vytvoření a zapnutí virtuálního python prostředí
- Instalace Python balíčků
- Kompilace Rust kódu
- Zapnutí serveru

#### Instalace Pythonu
Navštivte stránky [Pythonu](https://www.python.org/downloads/) a stáhněte nejnovější stabilní verzi pro vaše zařízení.

#### Instalace Rustu
Navštivte stránky [Rustu](https://www.rust-lang.org/tools/install) a stáhněte nejnovější stabilní verzi pro vaše zařízení.

#### Vytvoření a zapnutí viruálního prostředí
- Běžte do adresáře app/
- napište "python -m venv backend-venv/ (toto vytvoří virtuální prostředí pythonu)
- Běžte do adresáře backend-venv/bin/ a aktivujte si virtuální prostředí pomocí skriptu (na linuxu příkazem "source activate")

#### Instalace Python balíčků
- Se zapntuým virtuálním prostředím napište "pip install django djangorestframework markdown django-cors-headers django-filter maturin"

#### Kompilace Rust kódu
- Se zapnutým virtuálním prostředím běžte do app/rsnet/ a napište "maturin develop"

#### Zapnutí serveru
- Běžte do app/backend/ složky a napište "python manage.py runserver"
## 2. Ovládání aplikace pomocí CLI
Pro vývojovou verzi:
### Frontend
Běžte do adresáře app/frontend/ a napište "npm run dev".
### Backend
Běžte do složky app/backend-venv/bin/ a aktivujte skript pro vaše zařízení.
Poté ve složce app/backend/ napište "python manage.py runserver"

## 3. Frontend

## 4. Backend
Backendová část aplikace využívá jazyky Python a Rust.
### Python
Python společne s frameworkem Django zde pracuje pouze jako backend. Django je zde využito jako ovladač pro databázi, samotné dotazy z frontendu jsou řešené pomocí django-rest-framework. To je knihovna, která přidává podporu pro využití standardu JSON na komunikaci mezi aplikacemi.

### Rust
Rust je zde použit společně s python balíčkem Maturin, který pomáhá zkompilovat kód z Rustu tak, aby ho mohl Python využít jako externí knihovnu. Díky kompilovanému kódu jsou poté výpočetní funkce rychlejší a účinnější.
