using Random
using StatsBase

function cont_run(time=10000, n=10000, λ=0.05, q=0.1)
    r = zeros(time)
    θ = zeros(n)
    pchange = zeros(n)
    for t = 1:time
        ε = randn()
        if ε > 0
            r[t] =  sum(<(ε), θ) / (λ * n)
        else
            r[t] =  -sum(<(-ε), θ) / (λ * n)
        end
        θ .= ifelse.(rand!(pchange) .< q, abs(r[t]), θ)
    end
    return kurtosis(r)
end