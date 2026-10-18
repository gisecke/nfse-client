//mostrar e oculcar os campos cnpj e cpf
const radiosCadastro = document.querySelectorAll('input[name="radiosTipoCadastro"]');
        const campoCpf = document.getElementById('campo-cpf');
        const campoCnpj = document.getElementById('campo-cnpj');
        const inputCpf = document.getElementById('cpf');
        const inputCnpj = document.getElementById('cnpj');
        const inscricaoMunicipal = document.getElementById('im');

        function checkInitialRadio(radios) {
            radios.forEach(radio => {
                if (radio.checked) {
                    if (radio.value === "cpf") {
                        campoCpf.classList.remove('d-none');
                        campoCnpj.classList.add('d-none');
                        campoCpf.required = true;
                        campoCnpj.required = false;
                    } else {
                        campoCnpj.classList.remove('d-none');
                        campoCpf.classList.add('d-none');
                        campoCnpj.required = true;
                        campoCpf.required = false;
                    }
                }
            });
        }

        checkInitialRadio(radiosCadastro);

        radiosCadastro.forEach(radio => {
            radio.addEventListener('change', (e) => {
                if (e.target.value === "cpf") {
                    campoCpf.classList.remove('d-none');
                    campoCnpj.classList.add('d-none');
                    inputCnpj.value = ''; // Limpa o campo ao ocultar
                    campoCpf.required = true;
                    campoCnpj.required = false;
                } else {
                    campoCnpj.classList.remove('d-none');
                    campoCpf.classList.add('d-none');
                    inputCpf.value = ''; // Limpa o campo ao ocultar
                    campoCnpj.required = true;
                    campoCpf.required = false;
                }
            });
        });

// --- 2. Funções de Formatação (Máscaras) ---
        function mascaraCpf(valor) {
            return valor
                .replace(/\D/g, '') // Remove tudo o que não é dígito
                .replace(/(\d{3})(\d)/, '$1.$2') // Coloca ponto após o 3º dígito
                .replace(/(\d{3})(\d)/, '$1.$2') // Coloca ponto após o 6º dígito
                .replace(/(\d{3})(\d{1,2})$/, '$1-$2'); // Coloca traço antes dos últimos 2 dígitos
        }

        function mascaraCnpj(valor) {
            return valor
                .replace(/^(\d{2})(\d)/, '$1.$2') // Coloca ponto após o 2º dígito
                .replace(/^(\d{2})\.(\d{3})(\d)/, '$1.$2.$3') // Coloca ponto após o 5º dígito
                .replace(/\.(\d{3})(\d)/, '.$1/$2') // Coloca barra após o 8º dígito
                .replace(/(\d{4})(\d{1,2})$/, '$1-$2'); // Coloca traço antes dos últimos 2 dígitos
        }

        function mascaraInscricaoMunicipal(valor) {
            return valor
            .replace(/\D/g, ''); // Remove tudo o que não é dígito
        }

        // --- 3. Aplicar as máscaras nos eventos de digitação ---
        inputCpf.addEventListener('input', (e) => {
            e.target.value = mascaraCpf(e.target.value);
        });

        inputCnpj.addEventListener('input', (e) => {
            e.target.value = mascaraCnpj(e.target.value);
        });
        inscricaoMunicipal.addEventListener('input', (e) => {
            e.target.value = mascaraInscricaoMunicipal(e.target.value);
        });
        